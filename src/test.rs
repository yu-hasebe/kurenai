use crate::{
    canvas::Canvas,
    game_loop::GameLoop,
    game_state::GameState,
    image::Image,
    key_event::KeyboardEvent,
    point::{Dot, Point},
    sprite::Sprite,
};
use num_traits::{NumAssign, ToPrimitive};
use std::{marker::PhantomData, rc::Rc};

struct TestGameState {
    data1: GameObject,
    data2: GameObject,
}

impl GameState<KeyboardEvent, Point<Dot, i64>, i64> for TestGameState {
    fn key_event(&mut self, _key_event: &KeyboardEvent) {}
    fn update(&mut self) {}
    fn draw(&self, _canvas: &Canvas) {}
}

impl TestGameState {
    fn new() -> Self {
        let html_image_element_rc =
            Rc::new(image::create_new_html_image_element(&[], "gif").unwrap());
        let image_rc = Rc::new(Image::new(rc, GamePoint::new(0, 0), GamePoint::new(32, 32)));
        Self {
            data1: GameObject::new(image_rc.clone()),
            data2: GameObject::new(image_rc.clone()),
        }
    }
}

struct GameObject {
    image: Rc<Image<Point<Dot, i64>, i64>>,
}

impl Sprite<GamePoint<Dot, i64>, i64> for GameObject {
    fn image(&self) -> &web_sys::HtmlImageElement {
        &self.image.source_image().clone()
    }
}

impl GameObject {
    fn new(image: Rc<Image>) -> Self {
        Self { image }
    }
}

struct GamePoint<T, U>
where
    U: Clone + Copy + NumAssign + ToPrimitive,
{
    unit: PhantomData<T>,
    x: U,
    y: U,
}

impl<T, U> Point<T, U> for GamePoint<T, U>
where
    U: Clone + Copy + NumAssign + ToPrimitive,
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
    let image_repository = ImageRepository::new();
    GameLoop::run(test_game_state, canvas.unwrap()).unwrap();
}
