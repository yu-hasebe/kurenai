use crate::{canvas::Canvas, image::ImageRepository, key_event::KeyEvent};

pub trait GameService {
    fn key_event(&self, _key_event: &KeyEvent) {}
    fn update(&self) {}
    fn draw(&self, _image_repository: &ImageRepository, _canvas: &Canvas) {}
}
