use crate::{
    canvas::Canvas,
    game_loop::GameLoop,
    game_state::GameState,
    image::{image_id::ImageId, image_repository::ImageRepository, Image},
    key_event::{KeyEvent, KeyboardEvent},
    point::{Dot, Point},
    sprite::Sprite,
};
use std::{marker::PhantomData, rc::Rc};

// TestGameState
struct TestGameState {
    data1: GameObject,
    data2: GameObject,
}

impl GameState<KeyboardEvent, GamePoint<Dot>> for TestGameState {
    fn key_event(&mut self, key_event: &KeyboardEvent) {
        if key_event.enter() {
            // Update data
        }
    }
    fn update(&mut self) {
        // Update data
    }
    fn draw(&self, image_repository: &ImageRepository<GamePoint<Dot>>, canvas: &Canvas) {
        self.data1
            .draw(image_repository, canvas, GamePoint::new(0, 0))
            .unwrap();
        self.data2
            .draw(image_repository, canvas, GamePoint::new(32, 32))
            .unwrap();
    }
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
    let canvas = Canvas::new("main-canvas", GamePoint::new(480, 480), "game-container").unwrap();

    // image_repository factory
    let image_repository = {
        let new_html_image_element_rc =
            Rc::new(Image::<GamePoint<Dot>>::create_new_html_image_element(&[], "gif").unwrap());
        let image_repository = ImageRepository::new();
        image_repository
            .save(Image::new(
                ImageId(0),
                new_html_image_element_rc.clone(),
                GamePoint::new(64, 32),
                GamePoint::new(32, 32),
            ))
            .unwrap();
        image_repository
            .save(Image::new(
                ImageId(1),
                new_html_image_element_rc.clone(),
                GamePoint::new(64, 64),
                GamePoint::new(32, 32),
            ))
            .unwrap();
        image_repository
    };

    GameLoop::run(test_game_state, image_repository, canvas).unwrap();
}
