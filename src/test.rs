use crate::{
    canvas::HtmlCanvas,
    dot::{Dot, DotCoord},
    game_loop::{GameLoop, HtmlGameLoop},
    game_state::GameState,
    image::HtmlImage,
    key_event::{HtmlKeyboardEvent, KeyEvent},
};
use std::cell::RefCell;
use std::rc::Rc;

struct HtmlGameState {
    data: i64,
    image: HtmlImage,
}

impl GameState<HtmlKeyboardEvent> for HtmlGameState {
    fn key_event(&mut self, key_event: &HtmlKeyboardEvent) {}
    fn update(&mut self) {}
    fn draw(&self, html_canvas: &HtmlCanvas) {}
}

impl HtmlGameState {
    fn new() -> Self {
        Self {
            data: 0,
            image: HtmlImage::new(&[], "gif"),
        }
    }
}

#[test]
#[should_panic]
fn main() {
    let html_game_state = HtmlGameState::new();
    let html_canvas = HtmlCanvas::new(
        "main-canvas",
        &Dot::new(DotCoord(480), DotCoord(480)),
        "game-container",
    );
    HtmlGameLoop::run(html_game_state, html_canvas);
}
