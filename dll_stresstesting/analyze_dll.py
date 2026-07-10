#!/usr/bin/env python3
"""analyze_dll.py — deep static analysis of a Windows PE (.dll/.exe) file.

Usage:
    python analyze_dll.py <path-to-dll> [--json] [--top N] [--full] [--no-demangle]

Requires:
    pip install pefile

Reports: file hashes + whole-file entropy, COFF/optional headers (decoded
flag bits), sections (permissions + per-section Shannon entropy), imports
(incl. delay-load, flagged system vs non-system), exports (incl. forwarders,
ordinal-only symbols, C++ demangling + per-owning-type breakdown), debug/PDB
info (CodeView RSDS: GUID/age/path), version resource, Rich header (linker/
compiler build fingerprint), TLS callback count, load config (CFG/SafeSEH),
Authenticode signature presence, .NET/CLR detection, and a heuristics
section flagging things relevant to load-time performance and security
posture (no ASLR, no PDB, oversized import/export tables, packed sections).

C++ name demangling uses DbgHelp's UnDecorateSymbolName via ctypes, so the
per-owning-type export breakdown is Windows-only; elsewhere it falls back to
grouping by the UHT Z_Construct_* naming convention only.

Architecture of this file, top to bottom:
  1. Flag/lookup tables translating raw PE integer constants into readable names.
  2. Small standalone helpers (hex formatting, flag decoding, entropy, demangling).
  3. One `get_*` extraction function per PE concept (headers, sections, imports,
     exports, debug info, TLS, load config, signature, version info, Rich header).
     Each one takes the `pefile.PE` object (and sometimes the raw file bytes) and
     returns a plain dict/list — no printing, no side effects — so the exact same
     data can drive both the text report and the Markdown report without duplicating
     any parsing logic.
  4. `build_report()` — calls every extraction function once and assembles the
     final report dict, including the derived `heuristics` list.
  5. Two independent renderers over that same report dict: `print_report()` (plain
     text to stdout) and `build_markdown()` (a Markdown document, optionally written
     to disk via `--md-out`).
  6. `main()` — argument parsing and wiring it all together.
"""

import argparse
import ctypes
import hashlib
import json
import math
import platform
import re
import struct
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

try:
    import pefile
except ImportError:
    print("This script requires the 'pefile' package. Install it with:\n    pip install pefile", file=sys.stderr)
    sys.exit(1)


# ── flag tables (from the PE/COFF spec) ─────────────────────────────────────
# These mirror the IMAGE_FILE_*, IMAGE_DLLCHARACTERISTICS_*, and IMAGE_SCN_*
# constants from <winnt.h>. pefile exposes the raw integer bitmasks
# (FILE_HEADER.Characteristics, OPTIONAL_HEADER.DllCharacteristics, and each
# section's .Characteristics) but doesn't decode them into names itself, so we
# do that ourselves via decode_flags() below.

# IMAGE_FILE_* — overall COFF file header characteristics (is this a DLL? a
# system file? stripped of debug info? etc).
FILE_HEADER_FLAGS = {
    0x0001: "RELOCS_STRIPPED", 0x0002: "EXECUTABLE_IMAGE", 0x0004: "LINE_NUMS_STRIPPED",
    0x0008: "LOCAL_SYMS_STRIPPED", 0x0010: "AGGRESSIVE_WS_TRIM", 0x0020: "LARGE_ADDRESS_AWARE",
    0x0080: "BYTES_REVERSED_LO", 0x0100: "32BIT_MACHINE", 0x0200: "DEBUG_STRIPPED",
    0x0400: "REMOVABLE_RUN_FROM_SWAP", 0x0800: "NET_RUN_FROM_SWAP", 0x1000: "SYSTEM",
    0x2000: "DLL", 0x4000: "UP_SYSTEM_ONLY", 0x8000: "BYTES_REVERSED_HI",
}

# IMAGE_DLLCHARACTERISTICS_* — the security/loader-behavior flags in the optional
# header. The ones that matter most for load-time/security posture: DYNAMIC_BASE
# (ASLR), NX_COMPAT (DEP), and GUARD_CF (Control Flow Guard) — all three are
# checked directly in build_heuristics() below.
DLL_CHARACTERISTICS_FLAGS = {
    0x0020: "HIGH_ENTROPY_VA", 0x0040: "DYNAMIC_BASE", 0x0080: "FORCE_INTEGRITY",
    0x0100: "NX_COMPAT", 0x0200: "NO_ISOLATION", 0x0400: "NO_SEH", 0x0800: "NO_BIND",
    0x1000: "APPCONTAINER", 0x2000: "WDM_DRIVER", 0x4000: "GUARD_CF", 0x8000: "TERMINAL_SERVER_AWARE",
}

# IMAGE_SCN_* — per-section characteristics. We only decode a subset here (the
# ones relevant to understanding what a section actually contains and whether
# it's readable/writable/executable); the full spec has many more (alignment
# hints, COMDAT flags, etc.) that aren't useful for this kind of report.
SECTION_FLAGS = {
    0x00000020: "CNT_CODE", 0x00000040: "CNT_INITIALIZED_DATA", 0x00000080: "CNT_UNINITIALIZED_DATA",
    0x02000000: "MEM_DISCARDABLE", 0x04000000: "MEM_NOT_CACHED", 0x08000000: "MEM_NOT_PAGED",
    0x10000000: "MEM_SHARED", 0x20000000: "MEM_EXECUTE", 0x40000000: "MEM_READ", 0x80000000: "MEM_WRITE",
}

# IMAGE_FILE_MACHINE_* — only the ones likely to actually show up in practice.
MACHINE_TYPES = {0x014c: "x86 (I386)", 0x8664: "x64 (AMD64)", 0x01c4: "ARM", 0xaa64: "ARM64", 0x0200: "IA64"}

# IMAGE_SUBSYSTEM_* — what environment this binary expects to run in.
SUBSYSTEMS = {1: "NATIVE", 2: "WINDOWS_GUI", 3: "WINDOWS_CUI", 7: "POSIX_CUI", 9: "WINDOWS_CE_GUI", 14: "EFI_APPLICATION"}

# Heuristic prefix match for "this is Windows/CRT plumbing every native binary
# needs, not a dependency this specific module chose to have." Used to split
# the imports table into "system" vs "module" so the interesting (your own
# code's) dependencies aren't buried in kernel32/vcruntime/api-ms-win-crt-*
# noise that shows up basically everywhere.
SYSTEM_DLL_RE = re.compile(
    r'^(api-ms-win|kernel32|user32|advapi32|ntdll|msvcrt|vcruntime|ucrtbase|ole32|oleaut32|'
    r'ws2_32|shell32|gdi32|comctl32|comdlg32|rpcrt4|sechost|bcrypt|crypt32|version|winmm|'
    r'd3d|dxgi|dbghelp)', re.IGNORECASE)

# Used by classify_export() to group a demangled C++ export by its owning
# class/struct — e.g. "public: __cdecl SomeClass::SomeMethod(...)" -> "SomeClass".
# This turns "here are 400 raw export names" into "here are the ~15 types that
# actually generated those 400 names," which is almost always the more useful
# way to look at a large export table.
OWNER_RE = re.compile(r'\b([A-Za-z_]\w*)::')

# Fallback grouping for one specific naming convention: Unreal Engine's header
# tool (UHT) generates free functions named "Z_Construct_UClass_<TypeName>" (and
# similarly for UPackage/UEnum/UScriptStruct) for reflection registration. Those
# don't have a "::" in them (they're plain functions, not methods), so OWNER_RE
# won't catch them — this pulls the actual type name back out of that specific
# naming pattern instead. Harmless no-op on non-Unreal binaries (just won't match).
ZCONSTRUCT_RE = re.compile(r'Z_Construct_U(?:Class|Package|Enum|ScriptStruct)_(?:[A-Za-z0-9_]+?_)??([A-Za-z0-9_]+)$')


# ── small helpers ────────────────────────────────────────────────────────

def sh(n):
    """Format an integer as an uppercase, zero-padded 8-hex-digit string (e.g. addresses/RVAs)."""
    return f"0x{n:08X}"


def decode_flags(value, table):
    """Return the list of human-readable names whose bit is set in `value`, per `table`."""
    return [name for bit, name in table.items() if value & bit]


def shannon_entropy(data):
    """Shannon entropy of a byte string, in bits (0.0 = all one byte value, 8.0 = perfectly
    uniform random bytes). Used both on the whole file and per-section: compressed, encrypted,
    or packed data tends to sit close to 8.0, while normal code/data sections are lower (code
    especially, since instruction encodings are far from uniformly distributed)."""
    if not data:
        return 0.0
    counts = Counter(data)
    length = len(data)
    return -sum((c / length) * math.log2(c / length) for c in counts.values())


def demangle_all(names):
    """C++ demangle a batch of raw (decorated) export names via DbgHelp's
    UnDecorateSymbolName, loaded through ctypes so this script has no compiled
    dependency. Windows-only: on other platforms (or if dbghelp.dll can't be
    loaded for some reason) this just returns each name mapped to itself, and
    callers degrade gracefully (classify_export() falls back to the
    Z_Construct_* heuristic, which doesn't need demangled text)."""
    if platform.system() != "Windows":
        return {n: n for n in names}
    try:
        dbghelp = ctypes.WinDLL("dbghelp.dll")
    except OSError:
        return {n: n for n in names}

    UNDNAME_COMPLETE = 0x0000  # fully undecorate: return type, calling convention, everything
    buf = ctypes.create_string_buffer(2048)  # reused across calls; UnDecorateSymbolName just overwrites it
    result = {}
    for n in names:
        try:
            # UnDecorateSymbolName takes a plain (non-wide) C string in and writes the
            # undecorated form into `buf`, returning the written length (0 on failure —
            # e.g. for a name that was never C++-mangled in the first place).
            length = dbghelp.UnDecorateSymbolName(n.encode("mbcs", errors="replace"), buf, len(buf), UNDNAME_COMPLETE)
            result[n] = buf.value.decode("utf-8", errors="replace") if length > 0 else n
        except Exception:
            result[n] = n
    return result


def classify_export(raw_name, demangled_name):
    """Heuristic grouping of a single export by its owning C++ type / UHT-generated symbol
    family. Tries the demangled "Class::Method" pattern first (works for any C++ codebase);
    falls back to the Unreal-specific Z_Construct_* naming convention (works even without
    demangling, since that substring survives intact inside the still-mangled raw name); and
    finally gives up with a generic bucket. This is what turns a raw export list into the
    "top contributors by owning type" table."""
    m = OWNER_RE.search(demangled_name or "")
    if m:
        return m.group(1)
    m2 = ZCONSTRUCT_RE.search(raw_name)
    if m2:
        return m2.group(1)
    return "<other>"


# ── section-by-section extraction ───────────────────────────────────────
# Every function below takes the pefile.PE object (and, where the parsed
# object model doesn't already expose what we need, the raw file bytes too)
# and returns plain dicts/lists — never prints anything itself. That keeps
# all PE-parsing knowledge in one place, shared identically by both the text
# and Markdown renderers further down.

def get_file_info(path, raw):
    """Whole-file identity: path, size, cryptographic hashes (for looking the file up
    elsewhere / diffing two builds), and whole-file Shannon entropy (a coarse "is this
    binary unusually packed/compressed overall" signal, refined per-section in get_sections)."""
    return {
        "path": str(path.resolve()),
        "size_bytes": len(raw),
        "md5": hashlib.md5(raw).hexdigest(),
        "sha1": hashlib.sha1(raw).hexdigest(),
        "sha256": hashlib.sha256(raw).hexdigest(),
        "entropy": round(shannon_entropy(raw), 3),
    }


def get_file_header(pe):
    """The COFF file header: target architecture, section count, build timestamp (0 means
    either a reproducible/deterministic build or a deliberately stripped timestamp), and the
    decoded characteristics flags (notably whether this is actually a DLL)."""
    fh = pe.FILE_HEADER
    return {
        "machine": MACHINE_TYPES.get(fh.Machine, sh(fh.Machine)),
        "number_of_sections": fh.NumberOfSections,
        "timestamp_utc": (datetime.fromtimestamp(fh.TimeDateStamp, tz=timezone.utc).isoformat()
                           if fh.TimeDateStamp else "0 (stripped / reproducible build)"),
        "characteristics_flags": decode_flags(fh.Characteristics, FILE_HEADER_FLAGS),
        "is_dll": bool(fh.Characteristics & 0x2000),  # IMAGE_FILE_DLL
    }


def get_optional_header(pe):
    """The (misleadingly named — it's mandatory for DLLs/EXEs) optional header: entry point,
    preferred load address, linker/OS/subsystem versions, and the security-relevant
    DllCharacteristics flags (ASLR/DEP/CFG — see build_heuristics). Also recomputes the PE
    checksum from the file bytes and compares it against the value stored in the header;
    mismatches are extremely common for locally-built dev binaries (the checksum is often
    only enforced/updated for drivers and a few other special cases) so this is informational,
    not necessarily a red flag."""
    oh = pe.OPTIONAL_HEADER
    try:
        computed_checksum = pe.generate_checksum()
        checksum_valid = computed_checksum == oh.CheckSum
    except Exception:
        computed_checksum, checksum_valid = None, None

    return {
        "entry_point_rva": sh(oh.AddressOfEntryPoint),
        "image_base": sh(oh.ImageBase),
        "size_of_image": oh.SizeOfImage,
        "size_of_headers": oh.SizeOfHeaders,
        "checksum_declared": sh(oh.CheckSum),
        "checksum_computed": sh(computed_checksum) if computed_checksum is not None else None,
        "checksum_valid": checksum_valid,
        "linker_version": f"{oh.MajorLinkerVersion}.{oh.MinorLinkerVersion}",
        "os_version": f"{oh.MajorOperatingSystemVersion}.{oh.MinorOperatingSystemVersion}",
        "subsystem": SUBSYSTEMS.get(oh.Subsystem, str(oh.Subsystem)),
        "subsystem_version": f"{oh.MajorSubsystemVersion}.{oh.MinorSubsystemVersion}",
        "dll_characteristics_flags": decode_flags(oh.DllCharacteristics, DLL_CHARACTERISTICS_FLAGS),
    }


def get_sections(pe):
    """One entry per PE section (.text, .rdata, .data, ...): name, address/size, a compact
    RWX permission string decoded from the Characteristics bitmask, and per-section Shannon
    entropy (pefile's SectionStructure.get_entropy() computes this straight from that
    section's raw bytes) — the single most useful field here for spotting packed/obfuscated
    content, since a normal .text/.rdata section has a fairly predictable entropy range and
    an outlier (see build_heuristics' 7.5 threshold) is worth a closer look."""
    out = []
    for s in pe.sections:
        name = s.Name.rstrip(b"\x00").decode("utf-8", errors="replace")
        out.append({
            "name": name,
            "virtual_address": sh(s.VirtualAddress),
            "virtual_size": s.Misc_VirtualSize,
            "raw_size": s.SizeOfRawData,
            # 0x40000000 = IMAGE_SCN_MEM_READ, 0x80000000 = IMAGE_SCN_MEM_WRITE,
            # 0x20000000 = IMAGE_SCN_MEM_EXECUTE (also present in SECTION_FLAGS above,
            # inlined here too since we want a fixed-width "RWX"/"R-X"/etc string rather
            # than the free-form flag-name list decode_flags() would produce).
            "permissions": ("R" if s.Characteristics & 0x40000000 else "-")
                          + ("W" if s.Characteristics & 0x80000000 else "-")
                          + ("X" if s.Characteristics & 0x20000000 else "-"),
            "entropy": round(s.get_entropy(), 3),
            "flags": decode_flags(s.Characteristics, SECTION_FLAGS),
        })
    return out


def get_imports(pe):
    """The import table: for each DLL this binary depends on, every function (or bare
    ordinal, if imported by number instead of name) it pulls from it, plus a system-vs-module
    tag from SYSTEM_DLL_RE so the "your code's actual dependencies" signal isn't drowned out
    by kernel32/vcruntime/etc noise that's present in essentially every native DLL."""
    result = []
    for entry in getattr(pe, "DIRECTORY_ENTRY_IMPORT", []):
        dll_name = entry.dll.decode("utf-8", errors="replace")
        funcs = [imp.name.decode("utf-8", errors="replace") if imp.name else f"#{imp.ordinal}"
                 for imp in entry.imports]
        result.append({
            "dll": dll_name,
            "function_count": len(funcs),
            "functions": funcs,
            "is_system": bool(SYSTEM_DLL_RE.match(dll_name)),
        })
    return result


def get_delay_imports(pe):
    """Delay-load imports: dependencies the binary declared but deferred resolving until
    first actual use (a linker-level, opt-in optimization — see the "delay-loading" section of
    the general DLL-loading write-up). Structurally similar to get_imports() but a separate,
    smaller directory; wrapped per-entry in try/except since delay-import parsing in pefile is
    a little less battle-tested than the regular import directory, and a malformed/unusual
    entry here shouldn't take down the rest of the report."""
    result = []
    for entry in getattr(pe, "DIRECTORY_ENTRY_DELAY_IMPORT", []):
        try:
            dll_name = entry.dll.decode("utf-8", errors="replace")
            funcs = [imp.name.decode("utf-8", errors="replace") if imp.name else f"#{imp.ordinal}"
                     for imp in entry.imports]
            result.append({"dll": dll_name, "function_count": len(funcs), "functions": funcs})
        except Exception:
            continue
    return result


def get_exports(pe, demangle, top):
    """The export table: every symbol this binary makes available to others. Each symbol is
    either a plain name, an ordinal-only entry (no name — see the ordinal-vs-name trade-off in
    the general write-up), or a forwarder (points at another DLL's export instead of code in
    this one). When demangling is enabled, also classifies every named export by owning
    type/symbol-family (see classify_export) and returns the `top` biggest contributors —
    this is what turns "here are 400 exports" into "here are the ~15 types responsible for
    them," which is almost always the more actionable view of a large export table."""
    export_dir = getattr(pe, "DIRECTORY_ENTRY_EXPORT", None)
    if export_dir is None:
        return {"count": 0, "symbols": [], "by_type_top": []}

    symbols = []
    raw_names = []
    for sym in export_dir.symbols:
        name = sym.name.decode("utf-8", errors="replace") if sym.name else None
        forwarder = sym.forwarder.decode("utf-8", errors="replace") if sym.forwarder else None
        symbols.append({"ordinal": sym.ordinal, "rva": sh(sym.address) if sym.address else None,
                         "name": name, "forwarder": forwarder})
        if name:
            raw_names.append(name)

    # Demangle everything in one batch up front (rather than per-symbol inline) so it's
    # obvious this is the one place the (comparatively expensive, ctypes-call-per-name)
    # demangling happens, and so --no-demangle only needs to skip this one call.
    demangled_map = demangle_all(raw_names) if demangle and raw_names else {}
    by_type = Counter()
    for s in symbols:
        if not s["name"]:
            by_type["<ordinal-only>"] += 1
            continue
        s["demangled"] = demangled_map.get(s["name"], s["name"])
        by_type[classify_export(s["name"], s["demangled"])] += 1

    return {"count": len(symbols), "symbols": symbols, "by_type_top": by_type.most_common(top)}


def get_pdb_info(pe, raw):
    """Parses the debug directory, and specifically decodes CodeView (type 2) entries in the
    common "RSDS"/PDB70 format: a 4-byte signature, a 16-byte GUID, a 4-byte "age" counter, and
    a null-terminated path to the matching .pdb file — this GUID+age pair is exactly what a
    debugger/crash-reporting tool uses to verify it has the *exact* matching PDB for this
    *exact* build (not just a same-named one from a different build). pefile doesn't decode
    this format itself, so we do it manually here from the raw file bytes (hence needing `raw`
    passed in, rather than relying only on the parsed pefile object model) — the debug
    directory just tells us where in the file to find this blob and how big it is
    (PointerToRawData / SizeOfData); the GUID's on-disk byte order is the usual mixed-endian
    Windows GUID layout (first three fields little-endian, last two big-endian/as-is)."""
    results = []
    for dbg in getattr(pe, "DIRECTORY_ENTRY_DEBUG", []):
        s = dbg.struct
        entry = {"type": s.Type, "size": s.SizeOfData, "timestamp": s.TimeDateStamp}
        if s.Type == 2 and s.SizeOfData >= 24:  # IMAGE_DEBUG_TYPE_CODEVIEW, big enough for a PDB70 header
            try:
                dbg_raw = raw[s.PointerToRawData: s.PointerToRawData + s.SizeOfData]
                sig = dbg_raw[:4]
                if sig == b"RSDS":  # PDB70 (modern format; "NB10"/PDB20 below is the legacy one)
                    guid_bytes = dbg_raw[4:20]
                    age = struct.unpack("<I", dbg_raw[20:24])[0]
                    path = dbg_raw[24:].split(b"\x00", 1)[0].decode("utf-8", errors="replace")
                    # GUID layout: Data1 (uint32 LE), Data2 (uint16 LE), Data3 (uint16 LE),
                    # then Data4 as 8 raw bytes printed as-is — this reassembles the standard
                    # "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" textual GUID form.
                    d1, d2, d3 = struct.unpack("<IHH", guid_bytes[:8])
                    d4 = guid_bytes[8:]
                    guid_str = f"{d1:08X}-{d2:04X}-{d3:04X}-{d4[0]:02X}{d4[1]:02X}-" + "".join(f"{b:02X}" for b in d4[2:])
                    entry.update({"format": "PDB70/RSDS", "guid": guid_str, "age": age, "pdb_path": path})
                elif sig == b"NB10":
                    entry.update({"format": "PDB20/NB10"})
            except Exception:
                # Debug-directory contents that don't match our assumptions shouldn't break
                # the whole report — just report the raw type/size and move on.
                pass
        results.append(entry)
    return results


def get_tls_callbacks(pe):
    """Counts Thread Local Storage callbacks — functions the loader runs *before* the entry
    point (DllMain), which makes them an easy-to-forget source of load-time cost or subtle
    ordering bugs (see the loader-lock discussion in the general write-up). The TLS directory
    only gives us the *address* of a callback pointer array in memory (AddressOfCallBacks,
    expressed as a virtual address, i.e. relative to wherever the image is loaded — not a
    file offset), so we convert that to an RVA by subtracting ImageBase, then walk the
    null-terminated array of pointers (4 bytes on x86, 8 on x64) via pe.get_data(), which
    reads bytes by RVA out of the mapped sections regardless of which section actually holds
    them. The 64-entry cap is just a sanity backstop against ever looping forever on a
    corrupted/adversarial file."""
    tls = getattr(pe, "DIRECTORY_ENTRY_TLS", None)
    if tls is None or not tls.struct.AddressOfCallBacks:
        return []
    ptr_size = 8 if pe.FILE_HEADER.Machine == 0x8664 else 4
    rva = tls.struct.AddressOfCallBacks - pe.OPTIONAL_HEADER.ImageBase
    callbacks, offset = [], 0
    try:
        while len(callbacks) < 64:
            data = pe.get_data(rva + offset, ptr_size)
            val = struct.unpack("<Q" if ptr_size == 8 else "<I", data)[0]
            if val == 0:  # the array is terminated by a null pointer
                break
            callbacks.append(sh(val))
            offset += ptr_size
    except Exception:
        pass
    return callbacks


def get_load_config(pe):
    """A handful of fields from the Load Config directory relevant to exploit-mitigation
    posture: the stack security cookie address (used by /GS buffer-overflow protection),
    SafeSEH's registered exception-handler count, and the Control Flow Guard flags. Note this
    is a *different* CFG signal than DLL_CHARACTERISTICS_FLAGS' GUARD_CF bit checked elsewhere:
    that optional-header bit means "the loader should enforce CFG for this module," while
    GuardFlags' IMAGE_GUARD_CF_INSTRUMENTED bit (0x100) means "this module's own code actually
    contains CFG check instructions" — a binary can, in principle, have one without the other,
    so both are worth surfacing rather than assuming they always agree."""
    lc_dir = getattr(pe, "DIRECTORY_ENTRY_LOAD_CONFIG", None)
    if lc_dir is None:
        return None
    lc = lc_dir.struct
    guard_flags = getattr(lc, "GuardFlags", None)
    return {
        "security_cookie": sh(lc.SecurityCookie) if getattr(lc, "SecurityCookie", None) else None,
        "se_handler_count": getattr(lc, "SEHandlerCount", None),
        "guard_flags": sh(guard_flags) if guard_flags is not None else None,
        "cfg_enabled": bool(guard_flags & 0x00000100) if guard_flags is not None else None,  # IMAGE_GUARD_CF_INSTRUMENTED
    }


def get_signature_info(pe):
    """Whether an Authenticode digital signature is present, purely by checking whether the
    IMAGE_DIRECTORY_ENTRY_SECURITY data directory is populated. Deliberately does *not*
    attempt to cryptographically verify the signature (chain of trust, revocation, etc.) —
    that's a much bigger job than this script is trying to do; this only answers "is there a
    signature blob attached at all," which is still a useful signal (e.g. for guessing whether
    antivirus/EDR products are likely to treat this binary as more or less trusted)."""
    try:
        for d in pe.OPTIONAL_HEADER.DATA_DIRECTORY:
            if d.name == "IMAGE_DIRECTORY_ENTRY_SECURITY":
                return {"present": d.VirtualAddress != 0 and d.Size != 0, "size": d.Size}
    except Exception:
        pass
    return {"present": False, "size": 0}


def is_dotnet(pe):
    """True if this PE has a populated COM Descriptor ("CLR header") data directory, i.e. it's
    a .NET assembly (or a mixed-mode native/.NET binary) rather than a pure native module."""
    try:
        for d in pe.OPTIONAL_HEADER.DATA_DIRECTORY:
            if d.name == "IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR":
                return d.VirtualAddress != 0
    except Exception:
        pass
    return False


def get_version_info(pe):
    """The VERSIONINFO resource, if present: both the free-form "StringFileInfo" key/value
    pairs (CompanyName, FileDescription, FileVersion as text, etc. — whatever the linker/build
    process chose to embed) and the fixed-format VS_FIXEDFILEINFO numeric version fields
    (packed as two 16-bit halves per 32-bit field, hence the ">> 16" / "& 0xFFFF" unpacking
    below). Both can be present, absent, or disagree with each other in practice (the string
    table is free-form and sometimes goes stale), so both are surfaced rather than picking one."""
    strings = {}
    for file_info_list in getattr(pe, "FileInfo", []):
        for fi in file_info_list:
            for st in getattr(fi, "StringTable", []):
                for k, v in st.entries.items():
                    key = k.decode("utf-8", errors="replace") if isinstance(k, bytes) else k
                    val = v.decode("utf-8", errors="replace") if isinstance(v, bytes) else v
                    strings[key] = val

    fixed = {}
    ffi_list = getattr(pe, "VS_FIXEDFILEINFO", None)
    if ffi_list:
        ffi = ffi_list[0]
        fixed["file_version"] = f"{ffi.FileVersionMS >> 16}.{ffi.FileVersionMS & 0xFFFF}.{ffi.FileVersionLS >> 16}.{ffi.FileVersionLS & 0xFFFF}"
        fixed["product_version"] = f"{ffi.ProductVersionMS >> 16}.{ffi.ProductVersionMS & 0xFFFF}.{ffi.ProductVersionLS >> 16}.{ffi.ProductVersionLS & 0xFFFF}"
    return {"strings": strings, "fixed": fixed}


def get_rich_header(raw):
    """Decode the (undocumented but stable) MSVC linker 'Rich' header: a build
    fingerprint of every compiler/linker tool version and object-file count
    that went into this binary. Returns None if absent (e.g. /Brepro builds).

    Format (all of this is reverse-engineered/community knowledge, not officially
    documented by Microsoft, but has been stable for a very long time):
      - Somewhere in the DOS-stub region of the file sits a "Rich" ASCII marker,
        immediately followed by a 4-byte XOR key.
      - Earlier in that same region sits a "DanS" marker, XORed with that same key
        (i.e. `some_dword ^ key == "DanS"` at the right offset) — this is the true
        start of the structure; "Rich" is only the terminator/key-holder.
      - Between "DanS" (plus 3 reserved zero dwords right after it) and "Rich",
        the bytes are a flat array of (CompId, Count) dword pairs, each XORed with
        the same key. CompId packs a tool/product identifier in its high 16 bits
        and a build number in its low 16 bits; Count is how many times that exact
        tool/version combination was used (e.g. how many object files it compiled).
      This is genuinely useful as a build fingerprint: two binaries built by the
      exact same toolchain/settings will tend to produce the same Rich header
      entries, which is a quick way to notice "this one dependency was clearly
      built differently from the rest" without access to build logs.
    """
    rich_off = raw.find(b"Rich")
    if rich_off == -1:
        return None
    try:
        # The 4 bytes immediately after the "Rich" marker are the XOR key used
        # to obfuscate the whole structure (for no strong reason beyond history/convention).
        key = struct.unpack("<I", raw[rich_off + 4:rich_off + 8])[0]
        dans_target = int.from_bytes(b"DanS", "little")

        # Scan backwards in 4-byte steps looking for the XORed "DanS" marker. Bounded to
        # 0x200 bytes before "Rich" since the DOS stub region is small and fixed-ish in size;
        # if we don't find it in that window, this file's Rich header (if any) doesn't match
        # the expected layout and we bail out rather than guess.
        dans_off, pos, min_pos = None, rich_off - 4, max(0, rich_off - 0x200)
        while pos >= min_pos:
            if struct.unpack("<I", raw[pos:pos + 4])[0] ^ key == dans_target:
                dans_off = pos
                break
            pos -= 4
        if dans_off is None:
            return None

        # Skip "DanS" itself (4 bytes) plus 3 reserved dwords that are always zero before
        # XORing (so they read back as exactly `key` after XOR) — the real entries start
        # 16 bytes after dans_off.
        entries, pos = [], dans_off + 16
        while pos + 8 <= rich_off:
            comp_id, count = (v ^ key for v in struct.unpack("<II", raw[pos:pos + 8]))
            entries.append({"prod_id": (comp_id >> 16) & 0xFFFF, "build_id": comp_id & 0xFFFF, "count": count})
            pos += 8
        return {"key": sh(key), "entries": entries}
    except Exception:
        # Any structural surprise here means "not the layout we expected" — safer to report
        # no Rich header than to guess and print garbage.
        return None


def build_heuristics(report):
    """Turns the already-collected report data into a short list of plain-English notes about
    things relevant to load-time performance or security posture. Deliberately conservative:
    every check here is a simple, explainable threshold (not a scoring system), so each note
    can be read on its own without needing the rest of the report for context. Thresholds
    (20 imported DLLs, 200 exports, 7.5 entropy) are rules of thumb from practical experience
    triaging real binaries, not hard technical limits — they're meant to flag "worth a closer
    look," not "definitely a problem."""
    notes = []
    dllchar = report["optional_header"]["dll_characteristics_flags"]
    if "DYNAMIC_BASE" not in dllchar:
        notes.append("No ASLR (DYNAMIC_BASE not set) -- image loads at a fixed/predictable base address.")
    if "NX_COMPAT" not in dllchar:
        notes.append("No DEP/NX_COMPAT -- pages default to executable.")
    if "GUARD_CF" not in dllchar:
        notes.append("No Control Flow Guard (GUARD_CF not set).")
    if not report["debug"]["entries"]:
        notes.append("No debug directory -- no embedded PDB reference in this binary.")
    if not report["signature"]["present"]:
        notes.append("Not digitally signed (no Authenticode certificate).")
    if report["tls"]["callback_count"] > 0:
        notes.append(f"{report['tls']['callback_count']} TLS callback(s) present -- these run before "
                      f"DllMain/StartupModule and can add to load latency.")
    if len(report["imports"]) > 20:
        notes.append(f"Large import table: {len(report['imports'])} imported DLLs -- "
                      f"check for unused Build.cs dependencies pulling in extra linkage.")
    if report["exports"]["count"] > 200:
        top = report["exports"]["by_type_top"]
        extra = f" ('{top[0][0]}' alone contributes {top[0][1]})" if top else ""
        notes.append(f"Large export table: {report['exports']['count']} symbols{extra}.")
    if report["sections"]:
        worst = max(report["sections"], key=lambda s: s["entropy"])
        if worst["entropy"] > 7.5:
            notes.append(f"Section '{worst['name']}' has high entropy ({worst['entropy']}) -- "
                          f"possibly packed/compressed/encrypted data.")
    return notes


def build_report(path, pe, raw, demangle, top):
    """Runs every extraction function exactly once and assembles the full report dict. This
    is the single source of truth both renderers (print_report / build_markdown) consume —
    neither renderer re-parses the PE object or re-derives anything beyond simple formatting,
    which keeps the text and Markdown outputs guaranteed to agree with each other."""
    tls_callbacks = get_tls_callbacks(pe)  # computed once and reused (count + full list) rather than calling twice
    report = {
        "file": get_file_info(path, raw),
        "file_header": get_file_header(pe),
        "optional_header": get_optional_header(pe),
        "sections": get_sections(pe),
        "imports": get_imports(pe),
        "delay_imports": get_delay_imports(pe),
        "exports": get_exports(pe, demangle, top),
        "debug": {"entries": get_pdb_info(pe, raw)},
        "tls": {"callback_count": len(tls_callbacks), "callbacks": tls_callbacks},
        "load_config": get_load_config(pe),
        "signature": get_signature_info(pe),
        "dotnet": is_dotnet(pe),
        "version_info": get_version_info(pe),
        "rich_header": get_rich_header(raw),
    }
    # Heuristics are derived from the report itself (not from pe/raw directly), so this has
    # to run last, after every other field above already exists.
    report["heuristics"] = build_heuristics(report)
    return report


# ── text report printer ─────────────────────────────────────────────────
# Plain stdout report. Mirrors build_markdown()'s section order/content below,
# just formatted for a terminal instead of a Markdown file.

def _header(title):
    print(f"\n=== {title} ===")


def print_report(report, full):
    f = report["file"]
    _header("FILE")
    print(f"  Path:     {f['path']}")
    print(f"  Size:     {f['size_bytes']:,} bytes")
    print(f"  Entropy:  {f['entropy']} (whole file, 0-8 scale)")
    print(f"  MD5:      {f['md5']}")
    print(f"  SHA1:     {f['sha1']}")
    print(f"  SHA256:   {f['sha256']}")

    fh, oh = report["file_header"], report["optional_header"]
    _header("HEADERS")
    print(f"  Machine:              {fh['machine']}")
    print(f"  Is DLL:               {fh['is_dll']}")
    print(f"  Timestamp (UTC):      {fh['timestamp_utc']}")
    print(f"  File characteristics: {', '.join(fh['characteristics_flags'])}")
    print(f"  Subsystem:            {oh['subsystem']} (v{oh['subsystem_version']})")
    print(f"  Linker version:       {oh['linker_version']}")
    print(f"  Image base:           {oh['image_base']}   Size of image: {oh['size_of_image']:,}")
    print(f"  Entry point RVA:      {oh['entry_point_rva']}")
    print(f"  Checksum declared:    {oh['checksum_declared']}  valid: {oh['checksum_valid']}")
    print(f"  DLL characteristics:  {', '.join(oh['dll_characteristics_flags']) or '(none)'}")

    _header(f"SECTIONS ({len(report['sections'])})")
    print(f"  {'Name':<10}{'Perms':<7}{'RawSize':>10}{'VirtSize':>10}{'Entropy':>9}")
    for s in report["sections"]:
        print(f"  {s['name']:<10}{s['permissions']:<7}{s['raw_size']:>10,}{s['virtual_size']:>10,}{s['entropy']:>9}")

    imports = report["imports"]
    n_funcs = sum(d["function_count"] for d in imports)
    _header(f"IMPORTS ({len(imports)} DLLs, {n_funcs} functions)")
    for d in imports:
        tag = "system" if d["is_system"] else "module"
        print(f"  [{tag:<6}] {d['dll']:<40} {d['function_count']} functions")
        if full:  # --full: also dump every individual imported function name, not just counts
            for fn in d["functions"]:
                print(f"             {fn}")

    if report["delay_imports"]:
        _header(f"DELAY-LOAD IMPORTS ({len(report['delay_imports'])} DLLs)")
        for d in report["delay_imports"]:
            print(f"  {d['dll']:<40} {d['function_count']} functions")

    ex = report["exports"]
    _header(f"EXPORTS ({ex['count']} symbols)")
    if ex["by_type_top"]:
        print("  Top contributors by owning type/symbol group:")
        for name, count in ex["by_type_top"]:
            print(f"    {count:>5}  {name}")
    if full:  # --full: also dump every individual export (ordinal, RVA, demangled name/forwarder)
        print()
        for s in ex["symbols"]:
            label = s.get("demangled", s["name"]) or f"(ordinal {s['ordinal']} only)"
            fwd = f"  -> forwarded to {s['forwarder']}" if s["forwarder"] else ""
            print(f"    #{s['ordinal']:<6} {s['rva'] or '':<12} {label}{fwd}")

    dbg = report["debug"]["entries"]
    _header(f"DEBUG / PDB ({len(dbg)} entries)")
    for d in dbg:
        if "pdb_path" in d:
            print(f"  Format: {d['format']}  GUID: {d['guid']}  Age: {d['age']}")
            print(f"  PDB path: {d['pdb_path']}")
        else:
            print(f"  Type {d['type']} entry, {d['size']} bytes")

    _header("TLS")
    print(f"  Callback count: {report['tls']['callback_count']}")

    lc = report["load_config"]
    _header("LOAD CONFIG")
    if lc:
        print(f"  Security cookie: {lc['security_cookie']}   SEHandlerCount: {lc['se_handler_count']}")
        print(f"  Guard flags: {lc['guard_flags']}   CFG instrumented: {lc['cfg_enabled']}")
    else:
        print("  (not present)")

    _header("SIGNATURE / .NET")
    print(f"  Authenticode signature present: {report['signature']['present']}")
    print(f"  .NET / CLR (mixed-mode) assembly: {report['dotnet']}")

    vi = report["version_info"]
    if vi["strings"] or vi["fixed"]:
        _header("VERSION INFO")
        for k, v in vi["fixed"].items():
            print(f"  {k}: {v}")
        for k, v in vi["strings"].items():
            print(f"  {k}: {v}")

    rh = report["rich_header"]
    if rh:
        _header(f"RICH HEADER ({len(rh['entries'])} tool entries, key {rh['key']})")
        print("  ProdId  BuildId  Count")
        for e in rh["entries"]:
            print(f"  {e['prod_id']:>6}  {e['build_id']:>7}  {e['count']:>5}")

    _header("HEURISTICS")
    if report["heuristics"]:
        for note in report["heuristics"]:
            print(f"  ! {note}")
    else:
        print("  (nothing flagged)")
    print()


# ── markdown report writer ───────────────────────────────────────────────
# Same report dict, same section order as print_report() above, just emitted
# as Markdown (tables + collapsible <details> blocks for the potentially very
# long full import/export symbol dumps) instead of plain-text alignment.

def _md_escape(value):
    # Table cells like "<other>"/"<ordinal-only>" would otherwise be parsed as literal
    # (unrecognized, possibly stripped) HTML tags by strict Markdown renderers.
    return str(value).replace("<", "&lt;").replace(">", "&gt;")


def _md_table(headers, rows):
    """Build a GitHub-flavored-Markdown pipe table from a header row and a list of row lists."""
    lines = ["| " + " | ".join(headers) + " |", "|" + "|".join(["---"] * len(headers)) + "|"]
    lines += ["| " + " | ".join(_md_escape(c) for c in row) + " |" for row in rows]
    return "\n".join(lines)


def build_markdown(report, full):
    """Renders the full report as a single Markdown document (returned as a string; main()
    decides whether/where to write it to disk). `full` controls the same thing it does in
    print_report(): whether every individual import/export symbol gets listed (inside
    collapsible <details> blocks here, since a 400+ row table would otherwise dominate the
    whole document) or just the summary counts/tables."""
    f = report["file"]
    fh, oh = report["file_header"], report["optional_header"]
    lines = [f"# DLL Analysis: {Path(f['path']).name}", ""]

    lines += ["## File", "",
              f"- **Path**: `{f['path']}`",
              f"- **Size**: {f['size_bytes']:,} bytes",
              f"- **Entropy**: {f['entropy']} (whole file, 0-8 scale)",
              f"- **MD5**: `{f['md5']}`",
              f"- **SHA1**: `{f['sha1']}`",
              f"- **SHA256**: `{f['sha256']}`", ""]

    lines += ["## Headers", "",
              f"- **Machine**: {fh['machine']}",
              f"- **Is DLL**: {fh['is_dll']}",
              f"- **Timestamp (UTC)**: {fh['timestamp_utc']}",
              f"- **File characteristics**: {', '.join(fh['characteristics_flags'])}",
              f"- **Subsystem**: {oh['subsystem']} (v{oh['subsystem_version']})",
              f"- **Linker version**: {oh['linker_version']}",
              f"- **Image base**: {oh['image_base']}   **Size of image**: {oh['size_of_image']:,}",
              f"- **Entry point RVA**: {oh['entry_point_rva']}",
              f"- **Checksum declared**: {oh['checksum_declared']}  **valid**: {oh['checksum_valid']}",
              f"- **DLL characteristics**: {', '.join(oh['dll_characteristics_flags']) or '(none)'}", ""]

    lines += [f"## Sections ({len(report['sections'])})", "",
              _md_table(["Name", "Perms", "Raw Size", "Virtual Size", "Entropy"],
                        [[s["name"], s["permissions"], f"{s['raw_size']:,}", f"{s['virtual_size']:,}", s["entropy"]]
                         for s in report["sections"]]), ""]

    imports = report["imports"]
    n_funcs = sum(d["function_count"] for d in imports)
    lines += [f"## Imports ({len(imports)} DLLs, {n_funcs} functions)", "",
              _md_table(["DLL", "Kind", "Function Count"],
                        [[d["dll"], "system" if d["is_system"] else "module", d["function_count"]]
                         for d in imports]), ""]
    if full:
        # One collapsible block per imported DLL, so a module with hundreds of imported
        # functions doesn't force-expand the whole document by default.
        for d in imports:
            lines += [f"<details><summary>{d['dll']} — {d['function_count']} functions</summary>", "",
                      "```", *d["functions"], "```", "", "</details>", ""]

    if report["delay_imports"]:
        lines += [f"## Delay-load Imports ({len(report['delay_imports'])} DLLs)", "",
                  _md_table(["DLL", "Function Count"],
                            [[d["dll"], d["function_count"]] for d in report["delay_imports"]]), ""]

    ex = report["exports"]
    lines += [f"## Exports ({ex['count']} symbols)", ""]
    if ex["by_type_top"]:
        lines += ["### Top contributors by owning type/symbol group", "",
                  _md_table(["Count", "Type"], [[c, n] for n, c in ex["by_type_top"]]), ""]
    if full:
        # The full per-symbol table can easily run into the hundreds of rows for a large
        # module (see the 407-export example this script was originally built to inspect) —
        # collapsed by default for the same reason as the per-DLL import blocks above.
        rows = []
        for s in ex["symbols"]:
            label = s.get("demangled", s["name"]) or f"(ordinal {s['ordinal']} only)"
            if s["forwarder"]:
                label += f" -> forwarded to {s['forwarder']}"
            rows.append([s["ordinal"], s["rva"] or "", f"`{label}`"])
        lines += ["<details><summary>All export symbols</summary>", "",
                  _md_table(["Ordinal", "RVA", "Name"], rows), "", "</details>", ""]

    dbg = report["debug"]["entries"]
    lines += [f"## Debug / PDB ({len(dbg)} entries)", ""]
    for d in dbg:
        if "pdb_path" in d:
            lines += [f"- **Format**: {d['format']}  **GUID**: `{d['guid']}`  **Age**: {d['age']}",
                      f"- **PDB path**: `{d['pdb_path']}`"]
        else:
            lines.append(f"- Type {d['type']} entry, {d['size']} bytes")
    lines.append("")

    lines += ["## TLS", "", f"- **Callback count**: {report['tls']['callback_count']}", ""]

    lc = report["load_config"]
    lines += ["## Load Config", ""]
    if lc:
        lines += [f"- **Security cookie**: {lc['security_cookie']}   **SEHandlerCount**: {lc['se_handler_count']}",
                  f"- **Guard flags**: {lc['guard_flags']}   **CFG instrumented**: {lc['cfg_enabled']}", ""]
    else:
        lines += ["- (not present)", ""]

    lines += ["## Signature / .NET", "",
              f"- **Authenticode signature present**: {report['signature']['present']}",
              f"- **.NET / CLR (mixed-mode) assembly**: {report['dotnet']}", ""]

    vi = report["version_info"]
    if vi["strings"] or vi["fixed"]:
        lines += ["## Version Info", ""]
        for k, v in vi["fixed"].items():
            lines.append(f"- **{k}**: {v}")
        for k, v in vi["strings"].items():
            lines.append(f"- **{k}**: {v}")
        lines.append("")

    rh = report["rich_header"]
    if rh:
        lines += [f"## Rich Header ({len(rh['entries'])} tool entries, key {rh['key']})", "",
                  _md_table(["ProdId", "BuildId", "Count"],
                            [[e["prod_id"], e["build_id"], e["count"]] for e in rh["entries"]]), ""]

    lines += ["## Heuristics", ""]
    if report["heuristics"]:
        lines += [f"- ⚠ {note}" for note in report["heuristics"]]
    else:
        lines.append("- (nothing flagged)")
    lines.append("")

    return "\n".join(lines)


def write_markdown_report(report, out_dir, dll_path, full):
    """Renders and writes the Markdown report to `out_dir/<dll_stem>_dll_analysis.md`,
    creating the output directory (and any missing parents) if needed. Returns the path
    written, so the caller can print a confirmation message."""
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    out_file = out_dir / f"{dll_path.stem}_dll_analysis.md"
    out_file.write_text(build_markdown(report, full), encoding="utf-8")
    return out_file


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("dll_path", help="Path to the .dll (or .exe) to analyze")
    parser.add_argument("--json", action="store_true", help="Dump the full structured report as JSON instead of text")
    parser.add_argument("--top", type=int, default=25, help="How many export owning-type groups to show (default 25)")
    parser.add_argument("--full", action="store_true", help="In text mode, also list every import/export symbol")
    parser.add_argument("--no-demangle", action="store_true", help="Skip DbgHelp C++ demangling of export names")
    parser.add_argument("--md-out", metavar="FOLDER",
                         help="Also write a formatted Markdown report to FOLDER/<dll_name>_dll_analysis.md")
    args = parser.parse_args()

    path = Path(args.dll_path)
    if not path.is_file():
        print(f"error: file not found: {path}", file=sys.stderr)
        sys.exit(1)

    raw = path.read_bytes()
    try:
        # Parse from an in-memory buffer we own (rather than pe.__data__, which is an
        # mmap pefile may invalidate once its own parsing is done) so raw bytes stay
        # valid for our own file-level hashing/entropy/Rich-header/PDB scanning.
        pe = pefile.PE(data=raw)
    except pefile.PEFormatError as e:
        print(f"error: not a valid PE file: {e}", file=sys.stderr)
        sys.exit(1)

    report = build_report(path, pe, raw, demangle=not args.no_demangle, top=args.top)
    pe.close()  # release pefile's internal resources; we already have everything we need in `report`

    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print_report(report, full=args.full)

    if args.md_out:
        out_file = write_markdown_report(report, args.md_out, path, full=args.full)
        print(f"Markdown report written to: {out_file}")


if __name__ == "__main__":
    main()
