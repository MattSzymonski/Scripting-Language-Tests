using System.Runtime.InteropServices;

namespace Flappy;

/// <summary>
/// The native boundary. The Rust host resolves these three
/// <see cref="UnmanagedCallersOnlyAttribute"/> entry points by name
/// (<c>Flappy.Interop, game_cs</c>) and calls them each frame.
///
/// Bodies are wrapped in try/catch because a managed exception must never
/// unwind across the native call boundary.
/// </summary>
public static unsafe class Interop
{
    private static FlappyScene? _scene;

    [UnmanagedCallersOnly]
    public static void Init(EngineApi* api)
    {
        try
        {
            Engine.Bind(api);
            _scene = new FlappyScene();
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[game_cs] Init failed: {e}");
        }
    }

    [UnmanagedCallersOnly]
    public static void Update(float dt)
    {
        try
        {
            _scene?.Update(dt);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[game_cs] Update failed: {e}");
        }
    }

    [UnmanagedCallersOnly]
    public static void Draw()
    {
        try
        {
            _scene?.Draw();
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"[game_cs] Draw failed: {e}");
        }
    }
}
