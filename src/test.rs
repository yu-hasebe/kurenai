use crate::{
    canvas::HtmlCanvas,
    game_loop::{GameLoop, HtmlGameLoop},
    game_state::GameState,
    image::HtmlImage,
    key_event::HtmlKeyboardEvent,
};

struct HtmlGameState {
    _data: i64,
    _image: HtmlImage,
}

impl GameState<HtmlKeyboardEvent> for HtmlGameState {
    fn key_event(&mut self, _key_event: &HtmlKeyboardEvent) {}
    fn update(&mut self) {}
    fn draw(&self, _html_canvas: &HtmlCanvas) {}
}

impl HtmlGameState {
    fn new() -> Self {
        Self {
            _data: 0,
            _image: HtmlImage::new(&[], "gif"),
        }
    }
}

#[test]
#[should_panic]
fn main() {
    let html_game_state = HtmlGameState::new();
    let html_canvas = HtmlCanvas::new("main-canvas", 480, 480, "game-container");
    HtmlGameLoop::run(html_game_state, html_canvas);
}
