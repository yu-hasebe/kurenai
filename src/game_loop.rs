mod html_game_loop;
use crate::{game_state::GameState, key_event::KeyEvent};
use std::cell::RefCell;
use std::rc::Rc;

pub trait GameLoop<T, U>
where
    T: GameState<U>,
    U: KeyEvent,
{
    fn run(game_state_rc: Rc<RefCell<T>>, key_event_rc: Rc<RefCell<U>>);
}
