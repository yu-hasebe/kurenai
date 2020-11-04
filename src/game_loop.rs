use crate::game_service::GameService;
use crate::key_event::KeyEvent;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Debug)]
pub struct GameLoop;

impl GameLoop {
    pub fn run<T: 'static>(game_service: T)
    where
        T: GameService,
    {
        let game_service_rc = Rc::new(game_service);
        let key_event_rc = Rc::new(RefCell::new(KeyEvent::new()));
        KeyEvent::run(key_event_rc.clone());

        let callback_rc = Rc::new(RefCell::new(None));
        let callback = callback_rc.clone();
        *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let game_service = game_service_rc.clone();
            let key_event = key_event_rc.borrow();
            game_service.key_event(&key_event);
            game_service.update();
            game_service.draw();
            Self::request_animation_frame(
                callback_rc
                    .borrow()
                    .as_ref()
                    .expect("No Closure in this callback"),
            );
        }) as Box<dyn FnMut()>));
        Self::request_animation_frame(
            callback
                .borrow()
                .as_ref()
                .expect("No Closure in this callback"),
        );
    }
}

impl GameLoop {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .expect("No global window")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("Failed to call requestAnimationFrame");
    }
}
