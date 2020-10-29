use crate::{
    canvas::HtmlCanvas,
    game_loop::HtmlGameLoop,
    game_state::GameState,
    image::HtmlImage,
    key_event::HtmlKeyboardEvent,
    point::{Dot, Point},
};
use num_traits::{NumAssign, ToPrimitive};
use std::marker::PhantomData;
use std::string::ToString;

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

struct GamePoint<T, U>
where
    U: Clone + Copy + NumAssign + ToPrimitive + ToString,
{
    unit: PhantomData<T>,
    x: U,
    y: U,
}

impl<T, U> Point<T, U> for GamePoint<T, U>
where
    U: Clone + Copy + NumAssign + ToPrimitive + ToString,
{
    fn new(x: U, y: U) -> Self {
        Self {
            unit: PhantomData::<T>,
            x,
            y,
        }
    }

    fn x(&self) -> U {
        self.x
    }

    fn y(&self) -> U {
        self.y
    }
}

#[test]
#[should_panic]
fn main() {
    let html_game_state = HtmlGameState::new();
    let html_canvas = HtmlCanvas::new(
        "main-canvas",
        GamePoint::<Dot, i64>::new(480, 480),
        "game-container",
    );
    HtmlGameLoop::run(html_game_state, html_canvas);
}
