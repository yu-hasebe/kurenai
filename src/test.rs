use crate::{
    game_loop::{html_game_loop::HtmlGameLoop, GameLoop},
    game_state::GameState,
    graphic::{
        dot::{Dot, DotCoord},
        html_canvas::HtmlCanvas,
        html_image::HtmlImage,
    },
    key_event::{html_keyboard_event::HtmlKeyboardEvent, KeyEvent},
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
fn test() {
    let html_game_state_rc = Rc::new(RefCell::new(HtmlGameState::new()));
    let html_canvas_rc = Rc::new(HtmlCanvas::new(
        "main-canvas",
        &Dot::new(DotCoord(480), DotCoord(480)),
        "game-container",
    ));
    let html_keyboard_event_rc = Rc::new(RefCell::new(HtmlKeyboardEvent::new()));
    HtmlGameLoop::run(html_game_state_rc, html_keyboard_event_rc, html_canvas_rc);
}
