using System.Reflection;
using System.Runtime.Loader;

namespace Flappy.Loader;

/// <summary>
/// Owns a collectible <see cref="AssemblyLoadContext"/> for <c>game_cs.dll</c>
/// and forwards Init/Update/Draw into whichever version is currently loaded.
/// Polls the assembly's last-write time periodically and reloads when it
/// changes — the caller (<see cref="LoaderInterop"/>) doesn't need to know
/// anything happened.
///
/// Gameplay state resets on every reload: unloading the old context destroys
/// its whole managed heap, including <c>Flappy.Interop</c>'s static
/// <c>_scene</c> field. That's an accepted trade-off for this prototype — see
/// the repo's hot-reload docs for the alternative (keep state in
/// engine-owned memory instead of the managed heap).
///
/// Everything here runs on the single thread that drives the game loop
/// (Rust calls <c>Update</c>/<c>Draw</c> every frame from that thread), so
/// there's no locking or atomics to worry about — unlike the Rust hot-reload
/// path, which patches function pointers from a background watcher thread.
/// </summary>
internal sealed unsafe class GameHost
{
    private sealed class GameContext : AssemblyLoadContext
    {
        public GameContext() : base(isCollectible: true) { }

        protected override Assembly? Load(AssemblyName assemblyName) =>
            null; // Let the default context resolve shared BCL assemblies.
    }

    /// Check the file's timestamp roughly every half second rather than every
    /// frame — cheap, but avoids a syscall on every single frame.
    private const int PollEveryNFrames = 30;

    private readonly string _assemblyPath;

    private GameContext? _context;
    private delegate* unmanaged[Cdecl]<void*, void> _init;
    private delegate* unmanaged[Cdecl]<float, void> _update;
    private delegate* unmanaged[Cdecl]<void> _draw;

    private void* _api;
    private DateTime _lastWriteUtc;
    private int _frameCounter;

    public GameHost(string assemblyPath)
    {
        _assemblyPath = assemblyPath;
    }

    public void Init(void* api)
    {
        _api = api;
        Load();
    }

    public void Update(float dt)
    {
        if (++_frameCounter >= PollEveryNFrames)
        {
            _frameCounter = 0;
            MaybeReload();
        }

        if (_update != null)
        {
            _update(dt);
        }
    }

    public void Draw()
    {
        if (_draw != null)
        {
            _draw();
        }
    }

    private void MaybeReload()
    {
        DateTime written;
        try
        {
            written = File.GetLastWriteTimeUtc(_assemblyPath);
        }
        catch (IOException)
        {
            return; // File briefly missing/locked mid-build — try again later.
        }

        if (written <= _lastWriteUtc)
        {
            return;
        }

        try
        {
            Load();
            Console.WriteLine("[game_cs_loader] reloaded game_cs.dll");
        }
        catch (Exception e)
        {
            // Leave the previous (working) version in place.
            Console.Error.WriteLine($"[game_cs_loader] reload failed: {e}");
        }
    }

    private void Load()
    {
        _lastWriteUtc = File.GetLastWriteTimeUtc(_assemblyPath);

        var bytes = File.ReadAllBytes(_assemblyPath);
        var context = new GameContext();
        Assembly assembly;
        using (var stream = new MemoryStream(bytes))
        {
            assembly = context.LoadFromStream(stream);
        }

        var interopType = assembly.GetType("Flappy.Interop")
            ?? throw new InvalidOperationException($"Flappy.Interop not found in {_assemblyPath}");

        var init = (delegate* unmanaged[Cdecl]<void*, void>)GetExport(interopType, "Init");
        var update = (delegate* unmanaged[Cdecl]<float, void>)GetExport(interopType, "Update");
        var draw = (delegate* unmanaged[Cdecl]<void>)GetExport(interopType, "Draw");

        // Only swap over — and only unload the old context — once the new
        // one has fully resolved. If anything above throws, the previous
        // (working) version and its function pointers are left untouched.
        _context?.Unload();
        _context = context;
        _init = init;
        _update = update;
        _draw = draw;

        _init(_api);
    }

    private static nint GetExport(Type type, string methodName)
    {
        var method = type.GetMethod(methodName, BindingFlags.Public | BindingFlags.Static)
            ?? throw new MissingMethodException(type.FullName, methodName);
        return method.MethodHandle.GetFunctionPointer();
    }
}
