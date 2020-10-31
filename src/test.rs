use crate::{
    canvas::Canvas,
    game_loop::GameLoop,
    game_state::GameState,
    image::Image,
    key_event::KeyboardEvent,
    point::{Dot, Point},
};
use num_traits::{NumAssign, ToPrimitive};
use std::marker::PhantomData;
use std::string::ToString;

struct TestGameState {
    _data: i64,
    _image: Image,
}

impl GameState<KeyboardEvent> for TestGameState {
    fn key_event(&mut self, _key_event: &KeyboardEvent) {}
    fn update(&mut self) {}
    fn draw(&self, _canvas: &Canvas) {}
}

impl TestGameState {
    fn new() -> Self {
        Self {
            _data: 0,
            _image: Image::new(&[], "gif").unwrap(),
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

    fn x(&self) -> &U {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

#[test]
#[should_panic]
fn main() {
    let test_game_state = TestGameState::new();
    let canvas = Canvas::new(
        "main-canvas",
        GamePoint::<Dot, i64>::new(480, 480),
        "game-container",
    );
    GameLoop::run(test_game_state, canvas.unwrap()).unwrap();
}
