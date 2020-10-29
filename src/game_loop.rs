use crate::{canvas::HtmlCanvas, game_state::GameState, key_event::KeyEvent};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Debug)]
pub struct HtmlGameLoop;

impl HtmlGameLoop {
    pub fn run<T: 'static, U: 'static>(game_state: T, html_canvas: HtmlCanvas)
    where
        T: GameState<U>,
        U: KeyEvent,
    {
        let game_state_rc = Rc::new(RefCell::new(game_state));
        let html_canvas_rc = Rc::new(html_canvas);
        let key_event_rc = Rc::new(RefCell::new(U::new()));
        U::run(key_event_rc.clone());

        let closure_rc = Rc::new(RefCell::new(None));
        let closure = closure_rc.clone();
        *closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut game_state = game_state_rc.borrow_mut();
            let key_event = key_event_rc.borrow();
            let html_canvas = html_canvas_rc.clone();
            game_state.key_event(&key_event);
            game_state.update();
            game_state.draw(&html_canvas);
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
