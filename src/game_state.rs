use crate::{
    canvas::Canvas,
    image::image_repository::ImageRepository,
    key_event::KeyEvent,
    point::{Dot, Point},
};
use num_traits::{NumAssign, ToPrimitive};

pub trait GameState<T, U, V>
where
    T: KeyEvent,
    U: Point<Dot, V>,
    V: NumAssign + ToPrimitive,
{
    fn key_event(&mut self, _key_event: &T) {}
    fn update(&mut self) {}
    fn draw(&self, _image_repository: &ImageRepository<U, V>, _canvas: &Canvas) {}
}
