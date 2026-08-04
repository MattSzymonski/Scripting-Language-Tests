namespace Flappy;

/// <summary>The player-controlled bird: a circle pulled down by gravity that
/// flaps upward on input.</summary>
public sealed class Bird
{
    public Vec2 Pos;
    public float Vel;
    public readonly float Radius = Config.BirdRadius;
    public bool Dead;

    private float _rotation;

    public Bird(Vec2 start) => Pos = start;

    public void Flap() => Vel = Config.FlapVelocity;

    public void Update(float dt)
    {
        if (Engine.KeyPressed(Key.Space) || Engine.KeyPressed(Key.Up) || Engine.MouseLeftPressed())
        {
            Flap();
        }

        // Integrate gravity.
        Vel += Config.Gravity * dt;
        Pos = new Vec2(Pos.X, Pos.Y + Vel * dt);

        // Ceiling clamp.
        if (Pos.Y < Radius)
        {
            Pos = new Vec2(Pos.X, Radius);
            Vel = 0f;
        }

        // Ground is fatal.
        float floor = Engine.Height - Config.GroundHeight;
        if (Pos.Y + Radius >= floor)
        {
            Pos = new Vec2(Pos.X, floor - Radius);
            Dead = true;
        }

        // Tilt toward the direction of travel.
        float target = Math.Clamp(Vel / 600f, -0.5f, 1.4f);
        _rotation += (target - _rotation) * MathF.Min(10f * dt, 1f);
    }

    public void Draw()
    {
        // Body.
        Engine.DrawCircle(Pos.X, Pos.Y, Radius, Color.Gold);
        Engine.DrawCircleLines(Pos.X, Pos.Y, Radius, 2f, Color.Orange);

        // Beak: a triangle pointing the way the bird is tilted.
        var dir = new Vec2(MathF.Cos(_rotation), MathF.Sin(_rotation));
        var tip = Pos + dir * (Radius + 8f);
        var perp = dir.Perp() * (Radius * 0.4f);
        Engine.DrawTriangle(Pos + dir * Radius + perp, Pos + dir * Radius - perp, tip, Color.Orange);

        // Eye.
        Engine.DrawCircle(Pos.X + Radius * 0.35f, Pos.Y - Radius * 0.35f, Radius * 0.18f, Color.White);
        Engine.DrawCircle(Pos.X + Radius * 0.42f, Pos.Y - Radius * 0.35f, Radius * 0.09f, Color.Black);
    }

    public Rect? Bounds
    {
        get
        {
            float r = Radius * 0.85f;
            return new Rect(Pos.X - r, Pos.Y - r, r * 2f, r * 2f);
        }
    }
}
