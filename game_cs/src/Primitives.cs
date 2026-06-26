namespace Flappy;

/// <summary>A small 2D vector, enough for the game's needs.</summary>
public readonly struct Vec2(float x, float y)
{
    public readonly float X = x;
    public readonly float Y = y;

    public static Vec2 operator +(Vec2 a, Vec2 b) => new(a.X + b.X, a.Y + b.Y);
    public static Vec2 operator -(Vec2 a, Vec2 b) => new(a.X - b.X, a.Y - b.Y);
    public static Vec2 operator *(Vec2 a, float s) => new(a.X * s, a.Y * s);

    /// <summary>Perpendicular (rotated 90°), used to build the bird's beak.</summary>
    public Vec2 Perp() => new(-Y, X);
}

/// <summary>An axis-aligned rectangle used for collision tests.</summary>
public readonly struct Rect(float x, float y, float w, float h)
{
    public readonly float X = x;
    public readonly float Y = y;
    public readonly float W = w;
    public readonly float H = h;

    public bool Overlaps(in Rect o) =>
        X < o.X + o.W && X + W > o.X && Y < o.Y + o.H && Y + H > o.Y;
}

/// <summary>A packed <c>0xRRGGBBAA</c> colour matching the Rust unpacker.</summary>
public readonly struct Color
{
    public readonly uint Rgba;

    public Color(byte r, byte g, byte b, byte a = 255) =>
        Rgba = ((uint)r << 24) | ((uint)g << 16) | ((uint)b << 8) | a;

    public static Color FromFloat(float r, float g, float b, float a = 1f) =>
        new((byte)(r * 255), (byte)(g * 255), (byte)(b * 255), (byte)(a * 255));

    // Palette used by the game.
    public static readonly Color White = new(255, 255, 255);
    public static readonly Color Black = new(0, 0, 0);
    public static readonly Color Gold = new(255, 205, 40);
    public static readonly Color Orange = new(235, 140, 30);
    public static readonly Color Green = new(70, 190, 70);
    public static readonly Color DarkGreen = new(40, 130, 40);
    public static readonly Color Red = new(225, 60, 55);
    public static readonly Color Sky = new(115, 190, 240);
    public static readonly Color SkyDeep = new(100, 175, 235);
    public static readonly Color Ground = new(220, 190, 105);
    public static readonly Color GroundGrass = new(125, 200, 90);

    public Color WithAlpha(byte a) => new((byte)(Rgba >> 24), (byte)(Rgba >> 16), (byte)(Rgba >> 8), a);
}
