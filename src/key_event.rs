mod html_keyboard_event;
use std::cell::RefCell;
use std::rc::Rc;

pub trait KeyEvent {
    fn new() -> Self;
    fn run(key_event_rc: Rc<RefCell<Self>>);
    fn enter(&self) -> bool;
    fn arrow_left(&self) -> bool;
    fn arrow_up(&self) -> bool;
    fn arrow_right(&self) -> bool;
    fn arrow_down(&self) -> bool;
}
