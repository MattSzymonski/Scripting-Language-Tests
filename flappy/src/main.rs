//! Flappy Bird — a tiny game built on `mini_engine`.

mod bird;
mod config;
mod pipe;
mod scene;

use mini_engine::prelude::*;

use scene::FlappyScene;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird — mini_engine".to_owned(),
        window_width: config::WINDOW_WIDTH,
        window_height: config::WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    Engine::new()
        .with_clear_color(SKYBLUE)
        .run(FlappyScene::new())
        .await;
}
