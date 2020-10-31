use crate::{
    canvas::Canvas,
    image::image_repository::ImageRepository,
    key_event::KeyEvent,
    point::{Dot, Point},
};
use std::clone::Clone;

pub trait GameState<T, U>
where
    T: KeyEvent,
    U: Clone + Point<Dot>,
{
    fn key_event(&mut self, _key_event: &T) {}
    fn update(&mut self) {}
    fn draw(&self, _image_repository: &ImageRepository<U>, _canvas: &Canvas) {}
}
