use crate::{canvas::Canvas, key_event::KeyEvent};

pub trait GameState<T>
where
    T: KeyEvent,
{
    fn key_event(&mut self, _key_event: &T) {}
    fn update(&mut self) {}
    fn draw(&self, _canvas: &Canvas) {}
}
