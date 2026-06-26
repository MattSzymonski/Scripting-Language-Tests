namespace Flappy;

/// <summary>A vertical pipe pair scrolling left, with a gap to fly through.</summary>
public sealed class Pipe : GameObject
{
    public float X;
    public readonly float GapY;
    public readonly float Width = Config.PipeWidth;
    public readonly float Gap = Config.PipeGap;
    public bool Scored;

    public Pipe(float x, float gapY)
    {
        X = x;
        GapY = gapY;
    }

    /// <summary>The two solid rectangles (top and bottom) used for collision.</summary>
    public (Rect top, Rect bottom) Solids(float screenH)
    {
        float topH = GapY - Gap / 2f;
        float bottomY = GapY + Gap / 2f;
        float bottomH = screenH - Config.GroundHeight - bottomY;
        return (
            new Rect(X, 0f, Width, topH),
            new Rect(X, bottomY, Width, bottomH)
        );
    }

    public bool OffScreen => X + Width < 0f;

    public override bool IsAlive => !OffScreen;

    public override void Update(float dt) => X -= Config.ScrollSpeed * dt;

    public override void Draw()
    {
        var (top, bottom) = Solids(Engine.Height);
        foreach (var solid in new[] { top, bottom })
        {
            Engine.DrawRectangle(solid.X, solid.Y, solid.W, solid.H, Color.Green);
            Engine.DrawRectangleLines(solid.X, solid.Y, solid.W, solid.H, 4f, Color.DarkGreen);
        }

        // Classic wider "lip" at each side of the gap.
        const float lipH = 26f;
        const float overhang = 6f;
        float topLipY = GapY - Gap / 2f - lipH;
        float bottomLipY = GapY + Gap / 2f;
        foreach (float lipY in new[] { topLipY, bottomLipY })
        {
            Engine.DrawRectangle(X - overhang, lipY, Width + overhang * 2f, lipH, Color.Green);
            Engine.DrawRectangleLines(X - overhang, lipY, Width + overhang * 2f, lipH, 4f, Color.DarkGreen);
        }
    }
}
