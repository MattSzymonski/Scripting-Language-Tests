//! The per-frame context handed to the running scene.

/// Engine-provided state, refreshed once per frame and passed by reference
/// into [`Scene::update`](crate::scene::Scene::update) /
/// [`Scene::draw`](crate::scene::Scene::draw).
///
/// Both games in this repo read input and draw through their own `EngineApi`
/// function table instead (see `flappy/src/ffi.rs`), so this only carries
/// what the host loop itself needs: frame timing and the quit signal.
pub struct EngineContext {
    /// Seconds elapsed since the previous frame.
    pub dt: f32,

    pub(crate) should_quit: bool,
}

impl EngineContext {
    pub(crate) fn new() -> Self {
        Self {
            dt: 0.0,
            should_quit: false,
        }
    }

    /// Ask the engine to shut down after this frame.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
