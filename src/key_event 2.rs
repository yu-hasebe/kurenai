use crate::game_error::GameError;
use std::{cell::RefCell, rc::Rc};

pub trait KeyEvent {
    fn new() -> Self;
    fn run(key_event_rc: Rc<RefCell<Self>>) -> Result<(), GameError>;
    fn enter(&self) -> bool;
    fn arrow_left(&self) -> bool;
    fn arrow_up(&self) -> bool;
    fn arrow_right(&self) -> bool;
    fn arrow_down(&self) -> bool;
}
