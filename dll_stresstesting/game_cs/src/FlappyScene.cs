namespace Flappy;

/// <summary>
/// The whole game: a single scene that runs the Ready -> Playing -> GameOver
/// state machine, spawns pipes, and handles scoring and collisions.
/// </summary>
public sealed class FlappyScene
{
    private enum State { Ready, Playing, GameOver }

    private State _state = State.Ready;
    private Bird _bird;
    private readonly List<Pipe> _pipes = new();
    private float _spawnTimer;
    private int _score;
    private int _best;

    public FlappyScene()
    {
        _bird = new Bird(new Vec2(Config.BirdX, 720f * 0.45f));
    }

    private float FloorY => Engine.Height - Config.GroundHeight;

    private void Reset()
    {
        _state = State.Ready;
        _bird = new Bird(new Vec2(Config.BirdX, Engine.Height * 0.45f));
        _pipes.Clear();
        _spawnTimer = 0f;
        _score = 0;
    }

    private void SpawnPipe()
    {
        float gapY = Engine.RandRange(Config.GapMargin, FloorY - Config.GapMargin);
        _pipes.Add(new Pipe(Engine.Width, gapY));
    }

    public void Update(float dt)
    {
        if (Engine.KeyPressed(Key.Escape))
        {
            Engine.Quit();
            return;
        }

        bool flapped = Engine.KeyPressed(Key.Space) || Engine.KeyPressed(Key.Up) ||
                       Engine.MouseLeftPressed();

        switch (_state)
        {
            case State.Ready:
                // Gentle hover until the first flap.
                float baseY = Engine.Height * 0.45f;
                _bird.Pos = new Vec2(_bird.Pos.X, baseY + MathF.Sin((float)Engine.Time * 3f) * 10f);
                if (flapped)
                {
                    _state = State.Playing;
                    _bird.Flap();
                }
                break;

            case State.Playing:
                UpdatePlaying(dt);
                break;

            case State.GameOver:
                // Let the bird drop onto the ground.
                if (_bird.Pos.Y + _bird.Radius < FloorY)
                {
                    _bird.Vel += Config.Gravity * dt;
                    _bird.Pos = new Vec2(_bird.Pos.X, _bird.Pos.Y + _bird.Vel * dt);
                }
                if (flapped)
                    Reset();
                break;
        }
    }

    private void UpdatePlaying(float dt)
    {
        _bird.Update(dt);

        // Spawn pipes on a timer.
        _spawnTimer += dt;
        if (_spawnTimer >= Config.SpawnInterval)
        {
            _spawnTimer -= Config.SpawnInterval;
            SpawnPipe();
        }

        // Scroll and reap.
        foreach (var pipe in _pipes)
            pipe.Update(dt);
        _pipes.RemoveAll(p => !p.IsAlive);

        // Scoring + collision.
        float screenH = Engine.Height;
        Rect birdBox = _bird.Bounds!.Value;
        foreach (var pipe in _pipes)
        {
            if (!pipe.Scored && pipe.X + pipe.Width < _bird.Pos.X)
            {
                pipe.Scored = true;
                _score++;
            }

            var (top, bottom) = pipe.Solids(screenH);
            if (top.Overlaps(birdBox) || bottom.Overlaps(birdBox))
                _bird.Dead = true;
        }

        if (_bird.Dead)
        {
            _best = Math.Max(_best, _score);
            _state = State.GameOver;
        }
    }

    public void Draw()
    {
        DrawBackground();

        foreach (var pipe in _pipes)
            pipe.Draw();

        DrawGround();
        _bird.Draw();

        DrawHud();
    }

    // --- presentation ------------------------------------------------------

    private static void DrawBackground()
    {
        float w = Engine.Width;
        float h = Engine.Height;
        Engine.ClearBackground(Color.Sky);
        Engine.DrawRectangle(0, 0, w, h * 0.55f, Color.SkyDeep);

        (float cx, float cy, float r)[] clouds =
        {
            (90, 120, 26), (340, 80, 20), (250, 200, 18),
        };
        var cloud = Color.White.WithAlpha(217);
        foreach (var (cx, cy, r) in clouds)
        {
            Engine.DrawCircle(cx, cy, r, cloud);
            Engine.DrawCircle(cx + r, cy + 4, r * 0.8f, cloud);
            Engine.DrawCircle(cx - r, cy + 6, r * 0.7f, cloud);
        }
    }

    private static void DrawGround()
    {
        float w = Engine.Width;
        float top = Engine.Height - Config.GroundHeight;
        Engine.DrawRectangle(0, top, w, Config.GroundHeight, Color.Ground);
        Engine.DrawRectangle(0, top, w, 14f, Color.GroundGrass);
        Engine.DrawLine(0, top, w, top, 4f, Color.DarkGreen);
    }

    private void DrawHud()
    {
        float w = Engine.Width;

        // Score, centred near the top.
        string score = _score.ToString();
        float sw = Engine.MeasureTextWidth(score, 64f);
        float sx = (w - sw) / 2f;
        Engine.DrawText(score, sx + 2f, 84f, 64f, Color.Black.WithAlpha(100));
        Engine.DrawText(score, sx, 82f, 64f, Color.White);

        switch (_state)
        {
            case State.Ready:
                CenterText("FLAPPY BIRD", 0.30f, 56f, Color.Gold);
                CenterText("Press SPACE / click to flap", 0.42f, 26f, Color.White);
                CenterText("ESC to quit", 0.47f, 22f, Color.White);
                break;

            case State.GameOver:
                Engine.DrawRectangle(0, 0, w, Engine.Height, Color.Black.WithAlpha(90));
                CenterText("GAME OVER", 0.32f, 54f, Color.Red);
                CenterText($"Score: {_score}", 0.44f, 32f, Color.White);
                CenterText($"Best: {_best}", 0.50f, 28f, Color.Gold);
                CenterText("Press SPACE / click to retry", 0.60f, 24f, Color.White);
                break;
        }
    }

    private static void CenterText(string text, float yFrac, float size, Color color)
    {
        float x = (Engine.Width - Engine.MeasureTextWidth(text, size)) / 2f;
        float y = Engine.Height * yFrac;
        Engine.DrawText(text, x + 2f, y + 2f, size, Color.Black.WithAlpha(115));
        Engine.DrawText(text, x, y, size, color);
    }
}
