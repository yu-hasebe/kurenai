mod html_keyboard_event;

pub trait KeyEvent {
    fn enter(&self) -> bool;
    fn arrow_left(&self) -> bool;
    fn arrow_up(&self) -> bool;
    fn arrow_right(&self) -> bool;
    fn arrow_down(&self) -> bool;
}
