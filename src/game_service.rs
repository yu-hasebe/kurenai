use crate::key_event::KeyEvent;

/// Create a struct that implements this trait and pass it to the game_loop::run() function.
pub trait GameService {
    /// This function updates data based on key_event.
    fn key_event(&self, _key_event: &KeyEvent) {}

    /// This function updates data.
    fn update(&self) {}

    /// This function draws images on the canvas.
    fn draw(&self, _context: &web_sys::CanvasRenderingContext2d) {}
}
