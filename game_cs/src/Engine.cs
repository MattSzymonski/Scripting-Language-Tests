using System.Text;

namespace Flappy;

/// <summary>Integer key codes shared with the Rust <c>map_key</c> table.</summary>
public enum Key
{
    Space = 1,
    Up = 2,
    Down = 3,
    Left = 4,
    Right = 5,
    Escape = 6,
    Enter = 7,
}

public enum MouseButton
{
    Left = 0,
    Right = 1,
    Middle = 2,
}

/// <summary>
/// Friendly, idiomatic facade over the raw <see cref="EngineApi"/> table.
/// This is what game code uses, so the FFI details stay in one place.
/// </summary>
public static unsafe class Engine
{
    private static EngineApi _api;

    /// <summary>Capture the API table handed to us by the host (copied by value).</summary>
    public static void Bind(EngineApi* api) => _api = *api;

    // --- window / time -----------------------------------------------------
    public static float Width => _api.ScreenWidth();
    public static float Height => _api.ScreenHeight();
    public static double Time => _api.GetTime();

    // --- input -------------------------------------------------------------
    public static bool KeyPressed(Key key) => _api.KeyPressed((int)key) != 0;
    public static bool KeyDown(Key key) => _api.KeyDown((int)key) != 0;
    public static bool MousePressed(MouseButton button) => _api.MousePressed((int)button) != 0;
    public static Vec2 MousePosition => new(_api.MouseX(), _api.MouseY());

    // --- drawing -----------------------------------------------------------
    public static void ClearBackground(Color c) => _api.ClearBackground(c.Rgba);

    public static void DrawRectangle(float x, float y, float w, float h, Color c) =>
        _api.DrawRectangle(x, y, w, h, c.Rgba);

    public static void DrawRectangleLines(float x, float y, float w, float h, float thick, Color c) =>
        _api.DrawRectangleLines(x, y, w, h, thick, c.Rgba);

    public static void DrawCircle(float x, float y, float r, Color c) =>
        _api.DrawCircle(x, y, r, c.Rgba);

    public static void DrawCircleLines(float x, float y, float r, float thick, Color c) =>
        _api.DrawCircleLines(x, y, r, thick, c.Rgba);

    public static void DrawLine(float x1, float y1, float x2, float y2, float thick, Color c) =>
        _api.DrawLine(x1, y1, x2, y2, thick, c.Rgba);

    public static void DrawTriangle(Vec2 a, Vec2 b, Vec2 cc, Color color) =>
        _api.DrawTriangle(a.X, a.Y, b.X, b.Y, cc.X, cc.Y, color.Rgba);

    public static void DrawText(string text, float x, float y, float size, Color c)
    {
        fixed (byte* p = Utf8(text))
            _api.DrawText(p, x, y, size, c.Rgba);
    }

    public static float MeasureTextWidth(string text, float size)
    {
        fixed (byte* p = Utf8(text))
            return _api.MeasureTextWidth(p, size);
    }

    // --- misc --------------------------------------------------------------
    public static float RandRange(float min, float max) => _api.RandRange(min, max);
    public static void Quit() => _api.Quit();

    /// <summary>UTF-8 encode with a trailing NUL for the C string boundary.</summary>
    private static byte[] Utf8(string s)
    {
        int count = Encoding.UTF8.GetByteCount(s);
        var buffer = new byte[count + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buffer, 0);
        buffer[count] = 0;
        return buffer;
    }
}
