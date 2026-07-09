Co mierzy — 7 faz per moduł
Faza	Co to jest	Gdzie szukać problemu
1. DepEnumeration	Odczyt listy zależności z .Build.cs / .uplugin	Dużo zależności na raz → problem architektoniczny
2. DepLoading	Rekurencyjne ładowanie brakujących zależności	Tu najczęściej jest bottleneck — kaskadowe ładowanie
3. LocateDll	Wyszukanie pliku DLL na dysku	Network drive? Wolny HDD?
4. LoadLibrary	Windows LoadLibraryExW — mapowanie do pamięci, import resolution, DllMain, static init	Antywirus, wolny dysk, za dużo importów
5. GetSymbol	GetProcAddress("InitializeModule")	Praktycznie zawsze pomijalne
6. CreateInstance	Wywołanie InitializeModule() → tworzy obiekt modułu	Zwykle samo new — szybkie
7. StartupModule	IModuleInterface::StartupModule() — rejestracja typów, subsystemy, konfiguracja	Drugi największy killer — UObject registration, shader compilation
Output — dwa pliki
ModuleLoadProfile.csv — każdy moduł w osobnym wierszu:

→ Otwórz w Excelu, sortuj po Total_ms malejąco, od razu widzisz winowajców.

ModuleDependencyReport.txt — czytelny raport z:

TOP 10 najwolniejszych modułów
TOP 10 z największą ilością zależności
Procentowy breakdown: ile czasu w LoadLibrary vs Deps vs StartupModule
Automatyczna analiza z rekomendacjami
Jak użyć
Wklej kod z pliku do ModuleManager.cpp (oryginał zbackupuj)
Zakomentuj oryginalną LoadModuleWithCallback
W destruktorze FModuleManager dodaj SaveProfileToCSV(); SaveDependencyReport();
Przekompiluj edytor
Odpal → zamknij → otwórz CSV w Excelu

----------------




Masz pełną kontrolę nad kodem silnika, więc możesz to rozłożyć na części pierwsze. Oto konkretny plan ataku — od najprostszego do najbardziej szczegółowego:

Poziom 1: Unreal Insights (wbudowane, zero kodu)
Otwórz UnrealInsights.exe → zobaczysz timeline każdego załadowanego modułu. Od razu widać które pluginy są bottleneckiem i czy ładują się sekwencyjnie czy równolegle.

Poziom 2: Dodaj własny profiling do FModuleManager
Znajdź w kodzie silnika (masz do niego dostęp):

Dodaj timing do kluczowych funkcji:

To samo dla dependency resolution — znajdź gdzie silnik sprawdza PrivateDependencyModuleNames i dodaj timing per-zależność.

Poziom 3: Rozbij LoadModule na fazy
W ModuleManager.cpp, funkcja ładująca moduł robi kilka rzeczy — rozbij je:

Wrzuć to do CSV i otwórz w Excelu — sortuj po kolumnach, od razu widzisz co dominuje: czy to LoadLibrary (DllLoad), czy StartupModule (inicjalizacja), czy może dependency chain.

Poziom 4: Windows Performance Recorder (OS-level)
Zamknij wszystko, odpal tylko edytor, nagraj start:

Otwórz w Windows Performance Analyzer (wpa.exe) → zobaczysz:

Każde wywołanie LoadLibraryExW z czasem trwania
Które DLL-ki czekają na które (dependency chains)
I/O na dysku (czy bottleneck to SSD czy CPU)
Czy antywirus skanuje każdą DLL-kę (fltmgr.sys / MsMpEng.exe w stack trace)
To ci powie czy problem jest w Windows loaderze czy w kodzie Unreal.

Poziom 5: Dependency Walker — analiza statyczna
Użyj Dependencies (nowoczesna alternatywa Dependency Walkera):
https://github.com/lucasg/Dependencies

Otwórz jedną ze swoich plugin DLL → zobaczysz dokładne drzewo importów:

Pomnóż to ×200 i już wiesz dlaczego 90 sekund.

Poziom 6: ETW tracing — programistyczny
Jeśli chcesz zautomatyzować profilowanie, możesz w kodzie C++ silnika dodać ETW events:

Potem zbierasz ETW trace z dokładnością do mikrosekund.

Konkretny plan działania
Zacznij od Poziomu 2 — to 15 minut roboty, a da ci najwięcej insightu:

Dodaj timing do ModuleManager.cpp::LoadModuleWithCallback
Zapisz wyniki do CSV
Otwórz w Excelu, posortuj po LoadTime
Od razu zobaczysz:

Które 5 pluginów zajmuje 50% czasu
Czy czas jest zdominowany przez DLL load czy StartupModule
Czy pluginy ładują się sekwencyjnie (może da się zrównoleglić?)
Potem Poziom 4 (WPR) żeby zobaczyć czy Windows loader jest bottleneckem, czy kod Unreal.

Chcesz żebym ci przygotował konkretny patch do ModuleManager.cpp z pełnym timingiem i zapisem do CSV?

