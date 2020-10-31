use crate::{
    canvas::Canvas,
    game_loop::GameLoop,
    game_state::GameState,
    image::{image_id::ImageId, image_repository::ImageRepository},
    key_event::KeyboardEvent,
    point::{Dot, Point},
    sprite::Sprite,
};
use std::marker::PhantomData;

// TestGameState
struct TestGameState {
    data1: GameObject,
    data2: GameObject,
}

impl GameState<KeyboardEvent, GamePoint<Dot>> for TestGameState {
    fn key_event(&mut self, _key_event: &KeyboardEvent) {}
    fn update(&mut self) {}
    fn draw(&self, _image_repository: &ImageRepository<GamePoint<Dot>>, _canvas: &Canvas) {}
}

impl TestGameState {
    fn new() -> Self {
        Self {
            data1: GameObject::new(ImageId(0), GamePoint::new(32, 32)),
            data2: GameObject::new(ImageId(1), GamePoint::new(32, 32)),
        }
    }
}

// GameObject
struct GameObject {
    image_id: ImageId,
    size: GamePoint<Dot>,
}

impl Sprite<GamePoint<Dot>> for GameObject {
    fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    fn size(&self) -> &GamePoint<Dot> {
        &self.size
    }
}

impl GameObject {
    fn new(image_id: ImageId, size: GamePoint<Dot>) -> Self {
        Self { image_id, size }
    }
}

// GamePoint
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GamePoint<T> {
    unit: PhantomData<T>,
    x: i64,
    y: i64,
}

impl<T> Point<T> for GamePoint<T> {
    fn new(x: i64, y: i64) -> Self {
        Self {
            unit: PhantomData::<T>,
            x,
            y,
        }
    }

    fn x(&self) -> &i64 {
        &self.x
    }

    fn y(&self) -> &i64 {
        &self.y
    }
}

#[test]
#[should_panic]
fn main() {
    let test_game_state = TestGameState::new();
    let canvas = Canvas::new("main-canvas", GamePoint::new(480, 480), "game-container");
    let image_repository = ImageRepository::new();
    GameLoop::run(test_game_state, image_repository, canvas.unwrap()).unwrap();
}
