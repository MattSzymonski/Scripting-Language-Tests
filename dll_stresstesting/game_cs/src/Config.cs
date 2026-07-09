namespace Flappy;

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
