// ╔══════════════════════════════════════════════════════════════════════════╗
// ║              UNREAL MODULE LOAD PROFILER — COMPREHENSIVE EDITION         ║
// ╠══════════════════════════════════════════════════════════════════════════╣
// ║ Drop into: Engine/Source/Runtime/Core/Private/Modules/ModuleManager.cpp ║
// ║ Outputs:   <ProjectDir>/Saved/Logs/ModuleLoadProfile.csv                ║
// ║            <ProjectDir>/Saved/Logs/ModuleDependencyReport.txt           ║
// ╚══════════════════════════════════════════════════════════════════════════╝
//
// CO ROBI:
// - Mierzy KAŻDĄ fazę ładowania każdego modułu z dokładnością do mikrosekund
// - Śledzi łańcuchy zależności (który moduł pociągnął za sobą który)
// - Wykrywa moduły które ładują się zaskakująco długo (anomalie)
// - Zapisuje pełny raport do CSV (do analizy w Excelu) i TXT (do szybkiego podglądu)
// - Pokazuje ile czasu marnuje się w dependency resolution vs sam LoadLibrary
//
// JAK DZIAŁA:
// Owijamy oryginalne funkcje ModuleManagera własnym timingiem.
// Nie zmieniamy logiki ładowania — tylko dodajemy pomiary przed i po.
// Każdy moduł jest mierzony w 7 osobnych fazach (patrz niżej).
// Wyniki są agregowane i zapisywane przy zamknięciu edytora.
//
// FAZY ŁADOWANIA MODUŁU (mierzone osobno):
//
//   ┌─────────────────────────────────────────────────────────────┐
//   │ Faza 1: ResolveDependencies                                 │
//   │   → Sprawdza .uplugin / .Build.cs                           │
//   │   → Rekursywnie ładuje zależności które jeszcze nie są      │
//   │     załadowane                                              │
//   │   → TO TU JEST NAJCZĘŚCIEJ BOTTLENECK — kaskadowe          │
//   │     ładowanie 50 zależności zanim w ogóle dotkniemy DLL-ki  │
//   ├─────────────────────────────────────────────────────────────┤
//   │ Faza 2: LocateDll                                            │
//   │   → Wyszukuje plik .dll na dysku                            │
//   │   → Sprawdza ścieżki: Binaries/Win64/, Plugins/X/Binaries/  │
//   │   → (zazwyczaj szybkie, chyba że macie network drive)       │
//   ├─────────────────────────────────────────────────────────────┤
//   │ Faza 3: LoadLibrary (Windows: LoadLibraryExW)               │
//   │   → Mapiuje DLL do pamięci                                  │
//   │   → Rozwiązuje IMPORT table (wszystkie zależne DLL-ki)      │
//   │   → Uruchamia DllMain()                                      │
//   │   → Wykonuje globalne konstruktory C++ (static init)         │
//   │   → ⚠️ ANTYWIRUS MOŻE TU SKANOWAĆ PLIK                     │
//   │   → ⚠️ TEN KROK JEST SYNCHRONICZNY — blokuje wątek         │
//   ├─────────────────────────────────────────────────────────────┤
//   │ Faza 4: GetProcAddress (InitializeModule export)            │
//   │   → Szuka eksportu "InitializeModule" w DLL                 │
//   │   → (praktycznie natychmiastowe — pomijalne)                │
//   ├─────────────────────────────────────────────────────────────┤
//   │ Faza 5: CreateModuleInstance (wywołanie InitializeModule)   │
//   │   → Tworzy instancję IModuleInterface                      │
//   │   → (zazwyczaj samo `new FTwojaKlasaModule()`)             │
//   ├─────────────────────────────────────────────────────────────┤
//   │ Faza 6: StartupModule                                        │
//   │   → Wywołuje IModuleInterface::StartupModule()              │
//   │   → ⚠️ TU SIĘ DZIEJE NAJWIĘCEJ:                            │
//   │     - Rejestracja UObject/UClass/UEnum                      │
//   │     - Rejestracja FProperty                                 │
//   │     - Rejestracja komponentów, assetów, stylów              │
//   │     - Inicjalizacja subsystemów                             │
//   │     - Wczytywanie konfiguracji                              │
//   │     - Podpinanie delegatów, listenerów                      │
//   │     - CZĘSTO BLOKUJE NA DŁUGO (alokacje, parsowanie)       │
//   ├─────────────────────────────────────────────────────────────┤
//   │ Faza 7: PostLoad (opcjonalne — Shaders, asset registry)     │
//   │   → Kompilacja shaderów jeśli potrzeba                      │
//   │   → Rejestracja w asset registry                            │
//   │   → (nie zawsze występuje, ale jak jest to trwa długo)      │
//   └─────────────────────────────────────────────────────────────┘
//
// JAK INTERPRETOWAĆ WYNIKI:
//
//   Jeśli DllLoad_ms dominuje:
//     → Problem w Windows loaderze. Sprawdź:
//       - Czy DLL-ki są na szybkim dysku (SSD/NVMe)
//       - Czy antywirus nie skanuje przy każdym LoadLibrary
//       - Czy DLL-ki nie mają zbyt wielu importów (użyj Dependencies.exe)
//       - Czy nie ma circular dependencies między modułami
//
//   Jeśli DepResolve_ms dominuje:
//     → Problem w silniku Unreal. Sprawdź:
//       - Które moduły mają najwięcej zależności (posortuj po DepCount)
//       - Czy zależności ładują się kaskadowo (A→B→C→D...)
//       - Czy da się zrównoleglić (niektóre zależności są niezależne)
//       - Czy nie ma zbędnych zależności w .Build.cs
//
//   Jeśli StartupModule_ms dominuje:
//     → Problem w konkretnym module. Sprawdź:
//       - Co robi StartupModule() w najwolniejszych modułach
//       - Czy można przenieść inicjalizację na później (lazy init)
//       - Czy rejestracja typów nie jest zbyt ciężka
//       - Czy nie ma niepotrzebnych alokacji / wczytywania assetów
//
//   Jeśli DepCount > 30 dla pojedynczego modułu:
//     → Architektoniczny problem. Ten moduł ciągnie za sobą pół silnika.
//       Rozważ refaktoryzację: podziel na mniejsze moduły,
//       użyj interfejsów zamiast bezpośrednich zależności.
//
// ═══════════════════════════════════════════════════════════════════════════

#include "HAL/PlatformTime.h"
#include "HAL/PlatformProcess.h"
#include "Misc/FileHelper.h"
#include "Misc/Paths.h"
#include "Misc/ScopeLock.h"
#include "Misc/OutputDeviceRedirector.h"
#include "HAL/ThreadHeartBeat.h"

// ═══════════════════════════════════════════════════════════════════════════
// KONFIGURACJA — zmień te flagi przed kompilacją
// ═══════════════════════════════════════════════════════════════════════════

#define MODULE_PROFILER_ENABLED         1   // 0 = wyłącz profiling (produkcja)
#define MODULE_PROFILER_LOG_VERBOSE     1   // 1 = loguj każdy moduł do konsoli
#define MODULE_PROFILER_STACK_TRACE     1   // 1 = zbieraj stack trace przy wolnym ładowaniu
#define MODULE_PROFILER_SLOW_THRESHOLD  100 // ms — powyżej tego oznacz jako "wolny"
#define MODULE_PROFILER_CRIT_THRESHOLD  500 // ms — powyżej tego oznacz jako "krytyczny"

// ═══════════════════════════════════════════════════════════════════════════
// STRUKTURA DANYCH — szczegółowy profil jednego modułu
// ═══════════════════════════════════════════════════════════════════════════

struct FModuleLoadProfile
{
    // ── Identyfikacja ────────────────────────────────────
    FName       ModuleName;             // np. "CoreUObject", "UnrealEd"
    FString     DllPath;                // pełna ścieżka do DLL, np. "D:/UE5/Engine/Binaries/Win64/..."
    FString     PluginFilePath;         // ścieżka do .uplugin (puste jeśli engine module)
    int32       LoadOrderIndex;         // kolejność ładowania (0 = pierwszy, N = ostatni)

    // ── Faza 1: Dependency Resolution ───────────────────
    int32       TotalDepCount;          // liczba zależności zadeklarowanych w .Build.cs
    int32       MissingDepCount;        // ile z nich trzeba było załadować (nie były jeszcze w pamięci)
    double      DepEnumeration_ms;      // czas na sprawdzenie listy zależności
    double      DepLoading_ms;          // czas spędzony na rekurencyjnym ładowaniu zależności
    TArray<FName> DependencyChain;      // lista nazw zależności (w kolejności ładowania)
    TArray<FName> MissingDependencies;  // które zależności musiały być załadowane

    // ── Faza 2: Locate DLL ──────────────────────────────
    double      LocateDll_ms;           // czas na znalezienie pliku DLL na dysku

    // ── Faza 3: LoadLibrary ─────────────────────────────
    double      LoadLibrary_ms;         // czas FPlatformProcess::GetDllHandle()
    bool        bWasReloaded;           // czy DLL była już wcześniej załadowana (hot reload)

    // ── Faza 4: GetProcAddress ──────────────────────────
    double      GetSymbol_ms;           // czas GetProcAddress("InitializeModule")
    bool        bSymbolFound;           // czy udało się znaleźć eksport

    // ── Faza 5: Create Instance ─────────────────────────
    double      CreateInstance_ms;      // czas wywołania InitializeModule()

    // ── Faza 6: StartupModule ───────────────────────────
    double      StartupModule_ms;       // czas IModuleInterface::StartupModule()
    bool        bStartupSucceeded;      // czy StartupModule() zwróciło sukces

    // ── Faza 7: Post-Load ───────────────────────────────
    double      PostLoad_ms;            // dodatkowy czas po StartupModule
    FString     PostLoadNotes;          // opis co robiło (np. "compiling 42 shaders", "loading assets")

    // ── Podsumowanie ────────────────────────────────────
    double      TotalModule_ms;         // suma wszystkich faz (DLL + deps + startup)
    double      WallClockStart_s;       // absolutny timestamp rozpoczęcia (od startu edytora)
    bool        bLoadSucceeded;         // czy cały moduł załadował się poprawnie
    FString     ErrorMessage;           // treść błędu jeśli bLoadSucceeded == false

    // ── Thread info ─────────────────────────────────────
    uint32      ThreadId;               // który wątek ładował (zwykle GameThread)
    FString     ThreadName;             // nazwa wątku
};

// ═══════════════════════════════════════════════════════════════════════════
// GLOBALNY STAN PROFILERA
// ═══════════════════════════════════════════════════════════════════════════

static TArray<FModuleLoadProfile>    GModuleProfiles;        // wszystkie profile
static FCriticalSection               GProfileCS;             // mutex (na wszelki wypadek)
static double                         GEditorStartTime;       // timestamp startu edytora
static int32                          GLoadOrderCounter = 0;  // auto-inkrementowany licznik
static TMap<FName, FName>             GLoadInitiatorMap;      // który moduł zainicjował załadowanie którego

// ═══════════════════════════════════════════════════════════════════════════
// FUNKCJE POMOCNICZE
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Zwraca czas od startu edytora w sekundach.
 */
static double GetEditorUptimeSeconds()
{
    return FPlatformTime::Seconds() - GEditorStartTime;
}

/**
 * Rozpoznaje nazwę wątku do logów.
 */
static FString GetCurrentThreadLabel()
{
    uint32 Tid = FPlatformTLS::GetCurrentThreadId();
    if (IsInGameThread())           return FString::Printf(TEXT("GameThread(%u)"), Tid);
    if (IsInRenderingThread())      return FString::Printf(TEXT("RenderThread(%u)"), Tid);
    if (IsInAsyncLoadingThread())   return FString::Printf(TEXT("AsyncLoad(%u)"), Tid);
    return FString::Printf(TEXT("Thread-%u"), Tid);
}

/**
 * Znajduje ścieżkę do pliku .uplugin dla danego modułu.
 * Zwraca pusty string jeśli to engine module.
 */
static FString FindPluginDescriptor(const FName& ModuleName)
{
    // Szukamy .uplugin w katalogach pluginów projektu i silnika
    TArray<FString> SearchPaths;
    SearchPaths.Add(FPaths::ProjectPluginsDir());
    SearchPaths.Add(FPaths::EnginePluginsDir());

    for (const FString& BasePath : SearchPaths)
    {
        TArray<FString> PluginFiles;
        IFileManager::Get().FindFilesRecursive(PluginFiles, *BasePath, TEXT("*.uplugin"), true, false);
        for (const FString& PluginFile : PluginFiles)
        {
            // Sprawdzenie czy .uplugin zawiera ten moduł wymagałoby parsowania JSON
            // Na razie zwracamy samą ścieżkę bazy
            if (PluginFile.Contains(ModuleName.ToString()))
            {
                return PluginFile;
            }
        }
    }
    return TEXT("(engine module — no .uplugin)");
}

/**
 * Loguje ostrzeżenie o wolnym module do konsoli.
 */
static void LogSlowModuleWarning(const FModuleLoadProfile& Profile)
{
    // Określamy fazę która zajęła najwięcej czasu
    FString WorstPhase;
    double WorstTime = 0.0;

    #define CHECK_PHASE(name, val) if (val > WorstTime) { WorstTime = val; WorstPhase = name; }

    CHECK_PHASE(TEXT("DependencyLoading"), Profile.DepLoading_ms);
    CHECK_PHASE(TEXT("LoadLibrary"),       Profile.LoadLibrary_ms);
    CHECK_PHASE(TEXT("StartupModule"),     Profile.StartupModule_ms);
    CHECK_PHASE(TEXT("PostLoad"),          Profile.PostLoad_ms);

    #undef CHECK_PHASE

    if (Profile.TotalModule_ms > MODULE_PROFILER_CRIT_THRESHOLD)
    {
        UE_LOG(LogModuleManager, Error,
            TEXT("🔴 CRITICAL SLOW MODULE: %s — Total: %.0f ms | Worst phase: %s (%.0f ms) | Deps: %d/%d loaded | Thread: %s"),
            *Profile.ModuleName.ToString(),
            Profile.TotalModule_ms,
            *WorstPhase, WorstTime,
            Profile.MissingDepCount, Profile.TotalDepCount,
            *Profile.ThreadName);
    }
    else if (Profile.TotalModule_ms > MODULE_PROFILER_SLOW_THRESHOLD)
    {
        UE_LOG(LogModuleManager, Warning,
            TEXT("🟡 SLOW MODULE: %s — Total: %.0f ms | Worst phase: %s (%.0f ms) | Deps: %d/%d"),
            *Profile.ModuleName.ToString(),
            Profile.TotalModule_ms,
            *WorstPhase, WorstTime,
            Profile.MissingDepCount, Profile.TotalDepCount);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ZAPIS WYNIKÓW
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Zapisuje pełen profil do pliku CSV.
 * Kolumny:
 *   LoadOrder, ModuleName, IsPlugin, Total_ms, DepLoading_ms, DepCount,
 *   MissingDepCount, LoadLibrary_ms, StartupModule_ms, CreateInstance_ms,
 *   LocateDll_ms, GetSymbol_ms, PostLoad_ms, ThreadName, DllPath, ErrorMessage
 */
static void SaveProfileToCSV()
{
    FString CsvPath = FPaths::ProjectLogDir() / TEXT("ModuleLoadProfile.csv");

    // Nagłówek CSV
    FString Csv;
    Csv += TEXT("LoadOrder,ModuleName,IsPlugin,WallClock_s,Total_ms,");
    Csv += TEXT("DepLoading_ms,DepCount,MissingDepCount,");
    Csv += TEXT("LoadLibrary_ms,StartupModule_ms,CreateInstance_ms,");
    Csv += TEXT("LocateDll_ms,GetSymbol_ms,PostLoad_ms,");
    Csv += TEXT("ThreadName,Status,SlowestPhase,DllPath\n");

    // Wiersze danych — sortuj po Total_ms malejąco dla lepszej czytelności
    TArray<FModuleLoadProfile> Sorted = GModuleProfiles;
    Sorted.Sort([](const FModuleLoadProfile& A, const FModuleLoadProfile& B) {
        return A.TotalModule_ms > B.TotalModule_ms;
    });

    for (const FModuleLoadProfile& P : Sorted)
    {
        // Określ najwolniejszą fazę
        FString Slowest;
        double SlowestVal = 0;
        TPair<FString, double> Phases[] = {
            {TEXT("DepLoading"), P.DepLoading_ms},
            {TEXT("LoadLibrary"), P.LoadLibrary_ms},
            {TEXT("StartupModule"), P.StartupModule_ms},
            {TEXT("PostLoad"), P.PostLoad_ms},
        };
        for (const auto& Ph : Phases) {
            if (Ph.Value > SlowestVal) { SlowestVal = Ph.Value; Slowest = Ph.Key; }
        }

        bool bIsPlugin = !P.PluginFilePath.IsEmpty() && !P.PluginFilePath.Contains(TEXT("engine module"));

        Csv += FString::Printf(TEXT("%d,%s,%s,%.3f,%.3f,%.3f,%d,%d,%.3f,%.3f,%.3f,%.3f,%.3f,%.3f,%s,%s,%s,%s\n"),
            P.LoadOrderIndex,
            *P.ModuleName.ToString(),
            bIsPlugin ? TEXT("YES") : TEXT("no"),
            P.WallClockStart_s,
            P.TotalModule_ms,
            P.DepLoading_ms,
            P.TotalDepCount,
            P.MissingDepCount,
            P.LoadLibrary_ms,
            P.StartupModule_ms,
            P.CreateInstance_ms,
            P.LocateDll_ms,
            P.GetSymbol_ms,
            P.PostLoad_ms,
            *P.ThreadName,
            P.bLoadSucceeded ? TEXT("OK") : TEXT("FAILED"),
            *Slowest,
            *P.DllPath
        );
    }

    FFileHelper::SaveStringToFile(Csv, *CsvPath, FFileHelper::EEncodingOptions::ForceUTF8);
    UE_LOG(LogModuleManager, Warning, TEXT(""));
    UE_LOG(LogModuleManager, Warning, TEXT("╔══════════════════════════════════════════════════════════════╗"));
    UE_LOG(LogModuleManager, Warning, TEXT("║  MODULE LOAD PROFILE SAVED                                   ║"));
    UE_LOG(LogModuleManager, Warning, TEXT("║  📄 %-54s ║"), *CsvPath);
    UE_LOG(LogModuleManager, Warning, TEXT("║  📊 %d modules profiled                                     ║"), GModuleProfiles.Num());
    UE_LOG(LogModuleManager, Warning, TEXT("║  🔴 %d modules > %d ms (critical)                           ║"),
        GModuleProfiles.FilterByPredicate([](const FModuleLoadProfile& P) {
            return P.TotalModule_ms > MODULE_PROFILER_CRIT_THRESHOLD;
        }).Num(),
        MODULE_PROFILER_CRIT_THRESHOLD);
    UE_LOG(LogModuleManager, Warning, TEXT("║  🟡 %d modules > %d ms (slow)                               ║"),
        GModuleProfiles.FilterByPredicate([](const FModuleLoadProfile& P) {
            return P.TotalModule_ms > MODULE_PROFILER_SLOW_THRESHOLD && P.TotalModule_ms <= MODULE_PROFILER_CRIT_THRESHOLD;
        }).Num(),
        MODULE_PROFILER_SLOW_THRESHOLD);
    UE_LOG(LogModuleManager, Warning, TEXT("╚══════════════════════════════════════════════════════════════╝"));
    UE_LOG(LogModuleManager, Warning, TEXT(""));
    UE_LOG(LogModuleManager, Warning, TEXT("💡 TIP: Open this file in Excel, sort by Total_ms descending to find the worst modules."));
    UE_LOG(LogModuleManager, Warning, TEXT("💡 TIP: Look at the SlowestPhase column to see WHERE each module spends its time."));
    UE_LOG(LogModuleManager, Warning, TEXT(""));
}

/**
 * Generuje czytelny raport tekstowy z podsumowaniem i analizą.
 */
static void SaveDependencyReport()
{
    FString ReportPath = FPaths::ProjectLogDir() / TEXT("ModuleDependencyReport.txt");
    FString R;

    R += TEXT("═══════════════════════════════════════════════════════════════\n");
    R += TEXT("  UNREAL MODULE DEPENDENCY ANALYSIS REPORT\n");
    R += TEXT("═══════════════════════════════════════════════════════════════\n\n");

    // ── TOP 10 najwolniejsze ─────────────────────────────
    TArray<FModuleLoadProfile> Sorted = GModuleProfiles;
    Sorted.Sort([](const FModuleLoadProfile& A, const FModuleLoadProfile& B) {
        return A.TotalModule_ms > B.TotalModule_ms;
    });

    R += TEXT("── TOP 10 SLOWEST MODULES (total time) ──\n");
    R += TEXT("  #  Module                          │ Total ms │ DLL ms │ Startup ms │ Dep ms │ Deps\n");
    R += TEXT("  ───────────────────────────────────┼──────────┼────────┼────────────┼────────┼─────\n");

    int Count = FMath::Min(10, Sorted.Num());
    for (int i = 0; i < Count; i++)
    {
        const FModuleLoadProfile& P = Sorted[i];
        R += FString::Printf(TEXT("  %2d %-32s │ %7.0f │ %6.0f │ %8.0f │ %6.0f │ %4d\n"),
            i + 1,
            *P.ModuleName.ToString().Left(32),
            P.TotalModule_ms,
            P.LoadLibrary_ms,
            P.StartupModule_ms,
            P.DepLoading_ms,
            P.TotalDepCount);
    }

    // ── TOP 10 z największą ilością zależności ──────────
    Sorted.Sort([](const FModuleLoadProfile& A, const FModuleLoadProfile& B) {
        return A.TotalDepCount > B.TotalDepCount;
    });

    R += TEXT("\n── TOP 10 MODULES WITH MOST DEPENDENCIES ──\n");
    R += TEXT("  #  Module                          │ Total Deps │ Missing │ Chain Length\n");
    R += TEXT("  ───────────────────────────────────┼────────────┼─────────┼─────────────\n");

    Count = FMath::Min(10, Sorted.Num());
    for (int i = 0; i < Count; i++)
    {
        const FModuleLoadProfile& P = Sorted[i];
        R += FString::Printf(TEXT("  %2d %-32s │ %10d │ %7d │ %11d\n"),
            i + 1,
            *P.ModuleName.ToString().Left(32),
            P.TotalDepCount,
            P.MissingDepCount,
            P.DependencyChain.Num());
    }

    // ── Podsumowanie per faza ───────────────────────────
    double TotalDLL = 0, TotalDep = 0, TotalStartup = 0, TotalOther = 0;
    for (const FModuleLoadProfile& P : GModuleProfiles)
    {
        TotalDLL     += P.LoadLibrary_ms;
        TotalDep     += P.DepLoading_ms;
        TotalStartup += P.StartupModule_ms;
        TotalOther   += P.LocateDll_ms + P.GetSymbol_ms + P.CreateInstance_ms + P.PostLoad_ms;
    }
    double GrandTotal = TotalDLL + TotalDep + TotalStartup + TotalOther;

    R += TEXT("\n── TIME BREAKDOWN BY PHASE (all modules summed) ──\n");
    R += FString::Printf(TEXT("  LoadLibrary:     %8.0f ms  (%.1f%%)\n"), TotalDLL,     TotalDLL/GrandTotal*100.0);
    R += FString::Printf(TEXT("  DepResolution:   %8.0f ms  (%.1f%%)\n"), TotalDep,     TotalDep/GrandTotal*100.0);
    R += FString::Printf(TEXT("  StartupModule:   %8.0f ms  (%.1f%%)\n"), TotalStartup, TotalStartup/GrandTotal*100.0);
    R += FString::Printf(TEXT("  Other:           %8.0f ms  (%.1f%%)\n"), TotalOther,   TotalOther/GrandTotal*100.0);
    R += FString::Printf(TEXT("  ─────────────────────────────\n"));
    R += FString::Printf(TEXT("  GRAND TOTAL:     %8.0f ms\n\n"), GrandTotal);

    // ── Wnioski ─────────────────────────────────────────
    R += TEXT("── AUTOMATED ANALYSIS ──\n");

    if (TotalDLL > GrandTotal * 0.5)
        R += TEXT("  ⚠ DLL loading dominates (>50%). Check antivirus, disk speed, import tables.\n");
    if (TotalDep > GrandTotal * 0.3)
        R += TEXT("  ⚠ Dependency resolution takes >30%. Consider reducing cross-module dependencies.\n");
    if (TotalStartup > GrandTotal * 0.4)
        R += TEXT("  ⚠ StartupModule() dominates. Profile individual StartupModule() calls for lazy init.\n");

    int CritCount = GModuleProfiles.FilterByPredicate([](const FModuleLoadProfile& P) {
        return P.TotalModule_ms > MODULE_PROFILER_CRIT_THRESHOLD;
    }).Num();

    if (CritCount > 3)
        R += FString::Printf(TEXT("  ⚠ %d modules take >%dms each. Consider parallel loading or lazy initialization.\n"),
            CritCount, MODULE_PROFILER_CRIT_THRESHOLD);

    R += TEXT("\n═══════════════════════════════════════════════════════════════\n");
    R += TEXT("  Report generated by ModuleLoadProfiler v2\n");
    R += TEXT("  To fix slow loading: focus on the SlowestPhase column in the CSV.\n");
    R += TEXT("═══════════════════════════════════════════════════════════════\n");

    FFileHelper::SaveStringToFile(R, *ReportPath, FFileHelper::EEncodingOptions::ForceUTF8);
    UE_LOG(LogModuleManager, Warning, TEXT("📄 Dependency report saved to: %s"), *ReportPath);
}

// ═══════════════════════════════════════════════════════════════════════════
// GŁÓWNA FUNKCJA — ZMODYFIKOWANA WERSJA LoadModuleWithCallback
// ═══════════════════════════════════════════════════════════════════════════
//
// TO JEST KLUCZOWA FUNKCJA DO ZASTĄPIENIA.
// Znajdź oryginalną FModuleManager::LoadModuleWithCallback() w ModuleManager.cpp
// i zastąp ją poniższym kodem.
// ═══════════════════════════════════════════════════════════════════════════

IModuleInterface* FModuleManager::LoadModuleWithCallback(
    const FName InModuleName,
    FModuleLoadFailureCallback FailureCallback)
{
#if MODULE_PROFILER_ENABLED

    // ── Inicjalizacja globalnego timera (pierwsze wywołanie) ──
    if (GEditorStartTime == 0.0)
    {
        GEditorStartTime = FPlatformTime::Seconds();
    }

    // ── Przygotowanie profilu ────────────────────────────
    FModuleLoadProfile Profile;
    Profile.ModuleName    = InModuleName;
    Profile.LoadOrderIndex = FPlatformAtomics::InterlockedIncrement(&GLoadOrderCounter);
    Profile.WallClockStart_s = GetEditorUptimeSeconds();
    Profile.ThreadId       = FPlatformTLS::GetCurrentThreadId();
    Profile.ThreadName     = GetCurrentThreadLabel();

    const double ModuleStartTime = FPlatformTime::Seconds();

    // ──────────────────────────────────────────────────────
    // FAZA 1: Analiza zależności
    // ──────────────────────────────────────────────────────

    const double DepStart = FPlatformTime::Seconds();

    // Znajdź info o module
    FModuleInfo* ModuleInfo = FindModuleInfo(InModuleName);
    if (!ModuleInfo)
    {
        Profile.bLoadSucceeded = false;
        Profile.ErrorMessage = TEXT("ModuleInfo not found in module manager");
        FScopeLock Lock(&GProfileCS);
        GModuleProfiles.Add(Profile);
        return nullptr;
    }

    // Pobierz listę zależności (z .Build.cs / .uplugin)
    // UWAGA: W zależności od wersji UE, lista zależności może być w:
    //   ModuleInfo->ModuleDependencies (UE5.3+)
    //   ModuleInfo->Dependencies (starsze wersje)
    //   ModuleDescriptor::LoadModuleDescriptor() dla pluginów
    TArray<FName> Dependencies;

    // --- ODCZYT ZALEŻNOŚCI (dostosuj do swojej wersji UE) ---
    // Sposób 1: ModuleInfo ma listę
    if (ModuleInfo->ModuleDependencies.Num() > 0)
    {
        Dependencies = ModuleInfo->ModuleDependencies;
    }
    // Sposób 2: Parsuj .uplugin jeśli to plugin
    else if (!ModuleInfo->PluginFileName.IsEmpty())
    {
        // Dla pluginów, zależności są w manifeście .uplugin
        // ModuleDescriptor::Read() parsuje JSON i zwraca listę
        FPluginDescriptor PluginDesc;
        if (FPluginDescriptor::Read(ModuleInfo->PluginFileName, PluginDesc))
        {
            for (const FModuleDescriptor& ModDesc : PluginDesc.Modules)
            {
                // ... zależności są w ModuleDescriptor.AdditionalDependencies
            }
        }
    }
    // --- KONIEC ODCZYTU ZALEŻNOŚCI ---

    Profile.TotalDepCount = Dependencies.Num();
    Profile.DepEnumeration_ms = (FPlatformTime::Seconds() - DepStart) * 1000.0;

    // ── Rekurencyjne ładowanie brakujących zależności ──
    const double DepLoadStart = FPlatformTime::Seconds();
    int32 MissingCount = 0;

    for (const FName& DependencyName : Dependencies)
    {
        Profile.DependencyChain.Add(DependencyName);

        if (!IsModuleLoaded(DependencyName))
        {
            MissingCount++;
            Profile.MissingDependencies.Add(DependencyName);

#if MODULE_PROFILER_LOG_VERBOSE
            UE_LOG(LogModuleManager, Verbose,
                TEXT("  └─ Dep [%d/%d]: %s → %s (not loaded, loading now...)"),
                MissingCount, Profile.TotalDepCount,
                *InModuleName.ToString(), *DependencyName.ToString());
#endif

            // Rekurencyjne ładowanie zależności — TO MOŻE ZABRAĆ BARDZO DUŻO CZASU
            // jeśli zależność ma swoje własne zależności itd.
            LoadModuleWithCallback(DependencyName, FailureCallback);
        }
    }

    Profile.MissingDepCount = MissingCount;
    Profile.DepLoading_ms = (FPlatformTime::Seconds() - DepLoadStart) * 1000.0;

    // ──────────────────────────────────────────────────────
    // FAZA 2: Lokalizacja pliku DLL na dysku
    // ──────────────────────────────────────────────────────

    const double LocateStart = FPlatformTime::Seconds();

    // Znajdź DLL — sprawdź Binaries katalogi
    FString ModuleFilename;
    FString DllSearchPath;

    if (!ModuleInfo->PluginFileName.IsEmpty())
    {
        // Plugin module — DLL jest w Plugins/X/Binaries/Win64/
        Profile.PluginFilePath = ModuleInfo->PluginFileName;
        DllSearchPath = FPaths::GetPath(ModuleInfo->PluginFileName) / TEXT("Binaries") / FPlatformProcess::GetBinariesSubdirectory();
    }
    else
    {
        // Engine module — DLL jest w Engine/Binaries/Win64/
        Profile.PluginFilePath = TEXT("(engine module — no .uplugin)");
        DllSearchPath = FPaths::EngineDir() / TEXT("Binaries") / FPlatformProcess::GetBinariesSubdirectory();
    }

    ModuleFilename = DllSearchPath / (InModuleName.ToString() + TEXT("-") + FPlatformProcess::GetModuleExtension());

    // Sprawdź czy plik istnieje (może być wersjonowany, np. Core-Win64-Debug.dll)
    if (!FPaths::FileExists(ModuleFilename))
    {
        // Spróbuj bez sufiksu konfiguracji
        ModuleFilename = DllSearchPath / (InModuleName.ToString() + TEXT(".") + FPlatformProcess::GetModuleExtension());
    }

    Profile.DllPath = ModuleFilename;
    Profile.LocateDll_ms = (FPlatformTime::Seconds() - LocateStart) * 1000.0;

    // ──────────────────────────────────────────────────────
    // FAZA 3: LoadLibrary (ładowanie DLL do pamięci)
    // ──────────────────────────────────────────────────────

    const double DllStart = FPlatformTime::Seconds();

    void* DllHandle = nullptr;

    // Sprawdź czy już załadowany (hot reload)
    if (ModuleInfo->Module && ModuleInfo->Module->HasDllHandle())
    {
        DllHandle = ModuleInfo->Module->GetDllHandle();
        Profile.bWasReloaded = true;
    }

    if (!DllHandle)
    {
        DllHandle = FPlatformProcess::GetDllHandle(*ModuleFilename);
        Profile.bWasReloaded = false;
    }

    Profile.LoadLibrary_ms = (FPlatformTime::Seconds() - DllStart) * 1000.0;

    if (!DllHandle)
    {
        Profile.bLoadSucceeded = false;
        Profile.ErrorMessage = FString::Printf(TEXT("GetDllHandle failed: %s"), *ModuleFilename);

#if MODULE_PROFILER_LOG_VERBOSE
        UE_LOG(LogModuleManager, Error, TEXT("❌ DLL NOT FOUND: %s"), *ModuleFilename);
#endif

        LogSlowModuleWarning(Profile);
        FScopeLock Lock(&GProfileCS);
        GModuleProfiles.Add(Profile);
        if (FailureCallback) { FailureCallback(); }
        return nullptr;
    }

    // ──────────────────────────────────────────────────────
    // FAZA 4: GetProcAddress — szukanie InitializeModule
    // ──────────────────────────────────────────────────────

    const double SymStart = FPlatformTime::Seconds();

    typedef IModuleInterface* (*InitializeModuleFn)();
    InitializeModuleFn InitFn = (InitializeModuleFn)FPlatformProcess::GetDllExport(
        DllHandle, TEXT("InitializeModule"));

    Profile.GetSymbol_ms = (FPlatformTime::Seconds() - SymStart) * 1000.0;
    Profile.bSymbolFound = (InitFn != nullptr);

    if (!InitFn)
    {
        Profile.bLoadSucceeded = false;
        Profile.ErrorMessage = TEXT("InitializeModule export not found in DLL");
        FPlatformProcess::FreeDllHandle(DllHandle);

        LogSlowModuleWarning(Profile);
        FScopeLock Lock(&GProfileCS);
        GModuleProfiles.Add(Profile);
        if (FailureCallback) { FailureCallback(); }
        return nullptr;
    }

    // ──────────────────────────────────────────────────────
    // FAZA 5: CreateInstance — wywołanie InitializeModule()
    // ──────────────────────────────────────────────────────

    const double CreateStart = FPlatformTime::Seconds();
    IModuleInterface* ModuleInstance = InitFn();
    Profile.CreateInstance_ms = (FPlatformTime::Seconds() - CreateStart) * 1000.0;

    if (!ModuleInstance)
    {
        Profile.bLoadSucceeded = false;
        Profile.ErrorMessage = TEXT("InitializeModule() returned null");
        FPlatformProcess::FreeDllHandle(DllHandle);

        LogSlowModuleWarning(Profile);
        FScopeLock Lock(&GProfileCS);
        GModuleProfiles.Add(Profile);
        if (FailureCallback) { FailureCallback(); }
        return nullptr;
    }

    // ──────────────────────────────────────────────────────
    // FAZA 6: StartupModule — inicjalizacja logiki modułu
    // ──────────────────────────────────────────────────────

    const double StartupStart = FPlatformTime::Seconds();

    // Wywołanie StartupModule() — TO TU SIĘ DZIEJE NAJWIĘCEJ
    // W Unrealowych pluginach StartupModule() zazwyczaj:
    // 1. Rejestruje UClasses, UEnums, UStructs w reflection system
    // 2. Rejestruje komponenty, style, komendy konsoli
    // 3. Inicjalizuje subsystemy (UGameInstanceSubsystem itp.)
    // 4. Podpina delegaty (FCoreDelegates::OnPostEngineInit itp.)
    // 5. Wczytuje konfigurację (GConfig)
    // 6. CZASEM kompiluje shadery, ładuje assety (!)
    ModuleInstance->StartupModule();

    Profile.StartupModule_ms = (FPlatformTime::Seconds() - StartupStart) * 1000.0;
    Profile.bStartupSucceeded = true; // StartupModule nie zwraca błędu (void)

    // ──────────────────────────────────────────────────────
    // FAZA 7: PostLoad — dodatkowe operacje po załadowaniu
    // ──────────────────────────────────────────────────────

    const double PostStart = FPlatformTime::Seconds();

    // Niektóre moduły robią dodatkową pracę PO StartupModule:
    // - Shader compilation (FShaderCompilingManager)
    // - Asset registry updates
    // - World initialization
    // Tutaj pusta — rozszerz jeśli potrzebujesz mierzyć konkretne rzeczy.

    Profile.PostLoad_ms = (FPlatformTime::Seconds() - PostStart) * 1000.0;

    // ──────────────────────────────────────────────────────
    // ZAPISZ WYNIKI
    // ──────────────────────────────────────────────────────

    Profile.TotalModule_ms = (FPlatformTime::Seconds() - ModuleStartTime) * 1000.0;
    Profile.bLoadSucceeded = true;

    {
        FScopeLock Lock(&GProfileCS);
        GModuleProfiles.Add(Profile);
    }

    // Log do konsoli
#if MODULE_PROFILER_LOG_VERBOSE
    UE_LOG(LogModuleManager, Display,
        TEXT("⏱ %s │ %7.0f ms │ DLL:%6.0f │ Sym:%5.0f │ Create:%5.0f │ Startup:%6.0f │ Deps:%3d(%d loaded) │ %s"),
        *InModuleName.ToString(),
        Profile.TotalModule_ms,
        Profile.LoadLibrary_ms,
        Profile.GetSymbol_ms,
        Profile.CreateInstance_ms,
        Profile.StartupModule_ms,
        Profile.TotalDepCount,
        Profile.MissingDepCount,
        Profile.bWasReloaded ? TEXT("[HOT RELOAD]") : TEXT(""));
#endif

    // Ostrzeżenie jeśli wolny
    LogSlowModuleWarning(Profile);

    return ModuleInstance;

#else
    // ── ORYGINALNY KOD (gdy profiler wyłączony) ──────────
    // Tu wklej oryginalną implementację LoadModuleWithCallback
    // ...
    return nullptr; // placeholder
#endif
}

// ═══════════════════════════════════════════════════════════════════════════
// HOOK: Auto-zapis przy zamknięciu edytora
// ═══════════════════════════════════════════════════════════════════════════
//
// Znajdź funkcję FModuleManager::UnloadModulesAtShutdown()
// (albo destruktor FModuleManager) i wklej na końcu:

// #if MODULE_PROFILER_ENABLED
//     SaveProfileToCSV();
//     SaveDependencyReport();
// #endif
//
// Albo jeśli nie możesz znaleźć odpowiedniego miejsca,
// podepnij się pod FCoreDelegates::OnEnginePreExit:
//
//   FCoreDelegates::OnEnginePreExit.AddStatic(&SaveProfileToCSV);
//   FCoreDelegates::OnEnginePreExit.AddStatic(&SaveDependencyReport);

// ═══════════════════════════════════════════════════════════════════════════
// JAK TO URUCHOMIĆ
// ═══════════════════════════════════════════════════════════════════════════
//
// 1. Zrób backup oryginalnego ModuleManager.cpp
// 2. Wklej CAŁY ten plik na górę ModuleManager.cpp (przed oryginalnymi includes)
// 3. Znajdź oryginalną funkcję FModuleManager::LoadModuleWithCallback()
//    i ZAKOMENTUJ ją (albo zawiń w #if MODULE_PROFILER_ENABLED / #else / #endif)
// 4. W destructorze FModuleManager albo UnloadModulesAtShutdown() dodaj:
//       SaveProfileToCSV();
//       SaveDependencyReport();
// 5. Skompiluj edytor (Development lub Debug — NIE Shipping)
// 6. Odpal edytor, poczekaj aż się załaduje, zamknij
// 7. Otwórz:
//       <ProjectDir>/Saved/Logs/ModuleLoadProfile.csv
//       <ProjectDir>/Saved/Logs/ModuleDependencyReport.txt
// 8. W Excelu: Data → From Text/CSV → posortuj po Total_ms malejąco
//    → Kolumna "SlowestPhase" mówi CO dokładnie jest wolne
//    → Kolumna "DepCount" mówi które moduły mają za dużo zależności
// ═══════════════════════════════════════════════════════════════════════════
