use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub trait KeyEvent {
    fn new() -> Self;
    fn run(key_event_rc: Rc<RefCell<Self>>);
    fn enter(&self) -> bool;
    fn arrow_left(&self) -> bool;
    fn arrow_up(&self) -> bool;
    fn arrow_right(&self) -> bool;
    fn arrow_down(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct HtmlKeyboardEvent {
    enter: bool,
    arrow_left: bool,
    arrow_up: bool,
    arrow_right: bool,
    arrow_down: bool,
}

impl KeyEvent for HtmlKeyboardEvent {
    fn new() -> Self {
        Self {
            enter: false,
            arrow_left: false,
            arrow_up: false,
            arrow_right: false,
            arrow_down: false,
        }
    }

    fn run(key_event_rc: Rc<RefCell<Self>>) {
        let keydown_event_rc = key_event_rc.clone();
        let keydown_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            keydown_event_rc.borrow_mut().update_on_keydown(event);
        }) as Box<dyn FnMut(_)>);

        let keyup_event_rc = key_event_rc.clone();
        let keyup_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            keyup_event_rc.borrow_mut().update_on_keyup(event);
        }) as Box<dyn FnMut(_)>);

        Self::add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref());
        Self::add_event_listener_with_callback("keyup", keyup_handler.as_ref().unchecked_ref());

        keydown_handler.forget();
        keyup_handler.forget();
    }

    fn enter(&self) -> bool {
        self.enter
    }

    fn arrow_left(&self) -> bool {
        self.arrow_left
    }

    fn arrow_up(&self) -> bool {
        self.arrow_up
    }

    fn arrow_right(&self) -> bool {
        self.arrow_right
    }

    fn arrow_down(&self) -> bool {
        self.arrow_down
    }
}

impl HtmlKeyboardEvent {
    fn update_on_keydown(&mut self, event: web_sys::KeyboardEvent) {
        match event.key_code() {
            web_sys::KeyEvent::DOM_VK_RETURN => {
                self.enter = true;
            }
            web_sys::KeyEvent::DOM_VK_LEFT => {
                self.arrow_left = true;
            }
            web_sys::KeyEvent::DOM_VK_UP => {
                self.arrow_up = true;
            }
            web_sys::KeyEvent::DOM_VK_RIGHT => {
                self.arrow_right = true;
            }
            web_sys::KeyEvent::DOM_VK_DOWN => {
                self.arrow_down = true;
            }
            _ => {}
        }
    }

    fn update_on_keyup(&mut self, event: web_sys::KeyboardEvent) {
        match event.key_code() {
            web_sys::KeyEvent::DOM_VK_RETURN => {
                self.enter = false;
            }
            web_sys::KeyEvent::DOM_VK_LEFT => {
                self.arrow_left = false;
            }
            web_sys::KeyEvent::DOM_VK_UP => {
                self.arrow_up = false;
            }
            web_sys::KeyEvent::DOM_VK_RIGHT => {
                self.arrow_right = false;
            }
            web_sys::KeyEvent::DOM_VK_DOWN => {
                self.arrow_down = false;
            }
            _ => {}
        }
    }

    fn add_event_listener_with_callback(type_: &str, listener: &js_sys::Function) {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback(type_, listener)
            .unwrap();
    }
}
