#!/usr/bin/env bash
# graphify-init.sh
# One-shot script: installs prerequisites + graphify, then builds the graph for the current folder.
# Works on Windows (Git Bash / WSL), macOS, and Linux.
#
# Usage:
#   bash graphify-init.sh          # code-only (local AST, no API key)
#   bash graphify-init.sh --full   # include docs/PDFs/images (needs an API key set)

set -euo pipefail

FULL_MODE=false
if [[ "${1:-}" == "--full" ]]; then
    FULL_MODE=true
fi

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

say_ok()    { echo -e "${GREEN}  OK: $*${NC}"; }
say_warn()  { echo -e "${YELLOW}  WARN: $*${NC}"; }
say_info()  { echo -e "${CYAN}  $*${NC}"; }
say_err()   { echo -e "${RED}  ERROR: $*${NC}"; }
banner()    { echo -e "\n${CYAN}═══ $* ═══${NC}"; }

# ── 1. Python ────────────────────────────────────────────
banner "Checking Python"
PYTHON=""
for cmd in python3 python; do
    if command -v "$cmd" &>/dev/null; then
        PYTHON="$cmd"
        break
    fi
done

if [[ -z "$PYTHON" ]]; then
    say_err "Python 3.10+ is required but not found."
    echo "  Install from https://python.org (Windows) or your package manager."
    exit 1
fi

PY_VER=$("$PYTHON" --version 2>&1)
say_ok "$PY_VER"

# ── 2. uv ────────────────────────────────────────────────
banner "Checking uv"
if command -v uv &>/dev/null; then
    say_ok "uv $(uv --version)"
else
    say_warn "uv not found — installing..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    export PATH="$HOME/.local/bin:$PATH"
    if ! command -v uv &>/dev/null; then
        say_err "uv installation failed. Try manually: https://docs.astral.sh/uv/getting-started/installation/"
        exit 1
    fi
    say_ok "uv $(uv --version) installed"
fi

# ── 3. graphify ──────────────────────────────────────────
banner "Checking graphify"
export PATH="$HOME/.local/bin:$PATH"

if command -v graphify &>/dev/null; then
    say_ok "graphify $(graphify --version) already installed"
else
    say_warn "graphify not found — installing via uv..."
    uv tool install graphifyy
    if ! command -v graphify &>/dev/null; then
        say_err "graphify installation failed."
        exit 1
    fi
    say_ok "graphify $(graphify --version) installed"

    # Register the skill with AI assistants
    say_info "Registering skill with AI assistants..."
    graphify install 2>/dev/null || true
    graphify vscode install 2>/dev/null || true
    say_ok "Skill registered (Claude Code + VS Code Copilot Chat)"
fi

# ── 4. Build the graph ───────────────────────────────────
banner "Building knowledge graph"
TARGET_DIR="$(pwd)"
say_info "Target: $TARGET_DIR"

if $FULL_MODE; then
    say_warn "Mode: full (code + docs/PDFs/images — needs an API key set)"
    graphify .
else
    say_info "Mode: code-only (local AST, no API key needed)"
    graphify . --code-only
fi

# ── 5. Cluster & generate report ─────────────────────────
banner "Clustering & generating report"
graphify cluster-only . || say_warn "Clustering had issues (communities may be unlabeled — set an API key for named communities)"

# ── Done ─────────────────────────────────────────────────
echo ""
echo -e "${GREEN}═══ Done! ═══${NC}"
echo "  graphify-out/graph.json       — full graph data"
echo "  graphify-out/graph.html       — interactive visualization (open in browser)"
echo "  graphify-out/GRAPH_REPORT.md  — highlights & suggested questions"
echo ""
echo -e "${CYAN}Try:${NC}"
echo '  graphify query "what connects the engine to the scene?"'
echo "  graphify explain Engine"
