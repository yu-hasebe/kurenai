use crate::key_event::KeyEvent;

pub trait GameService {
    fn key_event(&self, _key_event: &KeyEvent) {}
    fn update(&self) {}
    fn draw(&self, _context: &web_sys::CanvasRenderingContext2d) {}
}
