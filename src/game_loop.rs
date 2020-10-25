pub mod html_game_loop;
use crate::{game_state::GameState, graphic::html_canvas::HtmlCanvas, key_event::KeyEvent};
use std::cell::RefCell;
use std::rc::Rc;

pub trait GameLoop<T, U>
where
    T: GameState<U>,
    U: KeyEvent,
{
    fn run(game_state_rc: Rc<RefCell<T>>, html_canvas_rc: Rc<HtmlCanvas>);
}
