using System.Runtime.InteropServices;

namespace Flappy.Loader;

/// <summary>
/// The stable native boundary. The Rust host resolves these
/// <see cref="UnmanagedCallersOnlyAttribute"/> entry points by name
/// (<c>Flappy.Loader.LoaderInterop, game_cs_loader</c>) instead of going
/// straight to <c>Flappy.Interop</c> in <c>game_cs.dll</c>.
///
/// Unlike <c>game_cs.dll</c>, this assembly is loaded once via hostfxr into
/// .NET's default (non-collectible) load context and never reloaded — so the
/// addresses Rust caches for <see cref="Init"/>/<see cref="Update"/>/
/// <see cref="Draw"/> stay valid forever. The actual game gets reloaded
/// underneath, inside <see cref="GameHost"/>, through its own collectible
/// <see cref="System.Runtime.Loader.AssemblyLoadContext"/>.
/// </summary>
public static unsafe class LoaderInterop
{
    private static GameHost? _host;

    [UnmanagedCallersOnly]
    public static void Init(void* api)
    {
        try
        {
            var dir = Environment.GetEnvironmentVariable("FLAPPY_MANAGED_DIR")
                ?? AppContext.BaseDirectory;
            var assemblyPath = Path.Combine(dir, "game_cs.dll");
            _host = new GameHost(assemblyPath);
            _host.Init(api);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[game_cs_loader] Init failed: {e}");
        }
    }

    [UnmanagedCallersOnly]
    public static void Update(float dt)
    {
        try
        {
            _host?.Update(dt);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[game_cs_loader] Update failed: {e}");
        }
    }

    [UnmanagedCallersOnly]
    public static void Draw()
    {
        try
        {
            _host?.Draw();
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[game_cs_loader] Draw failed: {e}");
        }
    }
}
