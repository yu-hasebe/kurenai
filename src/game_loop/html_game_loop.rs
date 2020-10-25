use crate::{game_loop::GameLoop, game_state::GameState, key_event::KeyEvent};
use std::cell::RefCell;
use std::rc::Rc;

pub struct HtmlGameLoop;

impl<T, U> GameLoop<T, U> for HtmlGameLoop
where
    T: GameState<U>,
    U: KeyEvent,
{
    fn run(game_state: Rc<RefCell<T>>, key_event: Rc<RefCell<U>>) {}
}
