use crate::{graphic::html_canvas::HtmlCanvas, key_event::KeyEvent};

pub trait GameState<T>
where
    T: KeyEvent,
{
    fn key_event(&mut self, key_event: &T) {}
    fn update(&mut self) {}
    fn draw(&self, html_canvas: &HtmlCanvas) {}
}
