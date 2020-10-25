use crate::{game_loop::GameLoop, game_state::GameState, key_event::KeyEvent};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct HtmlGameLoop;

// TODO: How to remove 'static?
impl<T: 'static, U: 'static> GameLoop<T, U> for HtmlGameLoop
where
    T: GameState<U>,
    U: KeyEvent,
{
    fn run(game_state_rc: Rc<RefCell<T>>, key_event_rc: Rc<RefCell<U>>) {
        U::run(key_event_rc.clone());

        let closure_rc = Rc::new(RefCell::new(None));
        let closure = closure_rc.clone();
        *closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut game_state = game_state_rc.borrow_mut();
            let key_event = key_event_rc.borrow();
            game_state.key_event(&key_event);
            game_state.update();
            game_state.draw();
            Self::request_animation_frame(closure_rc.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        Self::request_animation_frame(closure.borrow().as_ref().unwrap());
    }
}

impl HtmlGameLoop {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }
}
