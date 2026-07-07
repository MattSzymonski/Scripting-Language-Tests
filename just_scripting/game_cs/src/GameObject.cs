namespace Flappy;

/// <summary>
/// Base class for every entity, mirroring the Rust engine's <c>GameObject</c>
/// trait: override <see cref="Update"/> / <see cref="Draw"/> as needed.
/// </summary>
public abstract class GameObject
{
    public virtual void Update(float dt) { }
    public virtual void Draw() { }

    /// <summary>Return false to have the scene drop this object.</summary>
    public virtual bool IsAlive => true;

    /// <summary>Collision box, or null for objects with no physical presence.</summary>
    public virtual Rect? Bounds => null;
}

/// <summary>Tunable gameplay constants, all in one place.</summary>
public static class Config
{
    public const float GroundHeight = 96f;

    // Bird
    public const float BirdRadius = 18f;
    public const float Gravity = 1500f;
    public const float FlapVelocity = -470f;
    public const float BirdX = 120f;

    // Pipes
    public const float PipeWidth = 78f;
    public const float PipeGap = 185f;
    public const float ScrollSpeed = 175f;
    public const float SpawnInterval = 1.5f;
    public const float GapMargin = 90f;
}
