use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast};

/// This struct shows which key is down and which is up.
#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    enter: bool,
    arrow_left: bool,
    arrow_up: bool,
    arrow_right: bool,
    arrow_down: bool,
    digit_0: bool,
    digit_1: bool,
    digit_2: bool,
    digit_3: bool,
    digit_4: bool,
    digit_5: bool,
    digit_6: bool,
    digit_7: bool,
    digit_8: bool,
    digit_9: bool,
    key_a: bool,
    key_b: bool,
    key_c: bool,
    key_d: bool,
    key_e: bool,
    key_f: bool,
    key_g: bool,
    key_h: bool,
    key_i: bool,
    key_j: bool,
    key_k: bool,
    key_l: bool,
    key_m: bool,
    key_n: bool,
    key_o: bool,
    key_p: bool,
    key_q: bool,
    key_r: bool,
    key_s: bool,
    key_t: bool,
    key_u: bool,
    key_v: bool,
    key_w: bool,
    key_x: bool,
    key_y: bool,
    key_z: bool,
}

impl KeyEvent {
    /// This function creates a new instance. Pass it to KeyEvent::run().
    pub fn new() -> Self {
        Self {
            enter: false,
            arrow_left: false,
            arrow_up: false,
            arrow_right: false,
            arrow_down: false,
            digit_0: false,
            digit_1: false,
            digit_2: false,
            digit_3: false,
            digit_4: false,
            digit_5: false,
            digit_6: false,
            digit_7: false,
            digit_8: false,
            digit_9: false,
            key_a: false,
            key_b: false,
            key_c: false,
            key_d: false,
            key_e: false,
            key_f: false,
            key_g: false,
            key_h: false,
            key_i: false,
            key_j: false,
            key_k: false,
            key_l: false,
            key_m: false,
            key_n: false,
            key_o: false,
            key_p: false,
            key_q: false,
            key_r: false,
            key_s: false,
            key_t: false,
            key_u: false,
            key_v: false,
            key_w: false,
            key_x: false,
            key_y: false,
            key_z: false,
        }
    }

    /// This function calls add_event_listener_with_callback() and starts to serve KeyEvent.
    pub fn run(key_event_rc: Rc<RefCell<Self>>) {
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
}

impl KeyEvent {
    /// If the Enter key is down(up), this function returns true(false).
    pub fn enter(&self) -> bool {
        self.enter
    }

    /// If the ArrowLeft key is down(up), this function returns true(false).
    pub fn arrow_left(&self) -> bool {
        self.arrow_left
    }

    /// If the ArrowUp key is down(up), this function returns true(false).
    pub fn arrow_up(&self) -> bool {
        self.arrow_up
    }

    /// If the ArrowRight key is down(up), this function returns true(false).
    pub fn arrow_right(&self) -> bool {
        self.arrow_right
    }

    /// If the ArrowDown key is down(up), this function returns true(false).
    pub fn arrow_down(&self) -> bool {
        self.arrow_down
    }

    /// If the Digit0 key is down(up), this function returns true(false).
    pub fn digit_0(&self) -> bool {
        self.digit_0
    }

    /// If the Digit1 key is down(up), this function returns true(false).
    pub fn digit_1(&self) -> bool {
        self.digit_1
    }

    /// If the Digit2 key is down(up), this function returns true(false).
    pub fn digit_2(&self) -> bool {
        self.digit_2
    }

    /// If the Digit3 key is down(up), this function returns true(false).
    pub fn digit_3(&self) -> bool {
        self.digit_3
    }

    /// If the Digit4 key is down(up), this function returns true(false).
    pub fn digit_4(&self) -> bool {
        self.digit_4
    }

    /// If the Digit5 key is down(up), this function returns true(false).
    pub fn digit_5(&self) -> bool {
        self.digit_5
    }

    /// If the Digit6 key is down(up), this function returns true(false).
    pub fn digit_6(&self) -> bool {
        self.digit_6
    }

    /// If the Digit7 key is down(up), this function returns true(false).
    pub fn digit_7(&self) -> bool {
        self.digit_7
    }

    /// If the Digit8 key is down(up), this function returns true(false).
    pub fn digit_8(&self) -> bool {
        self.digit_8
    }

    /// If the Digit9 key is down(up), this function returns true(false).
    pub fn digit_9(&self) -> bool {
        self.digit_9
    }

    /// If the KeyA key is down(up), this function returns true(false).
    pub fn key_a(&self) -> bool {
        self.key_a
    }

    /// If the KeyB key is down(up), this function returns true(false).
    pub fn key_b(&self) -> bool {
        self.key_b
    }

    /// If the KeyC key is down(up), this function returns true(false).
    pub fn key_c(&self) -> bool {
        self.key_c
    }

    /// If the KeyD key is down(up), this function returns true(false).
    pub fn key_d(&self) -> bool {
        self.key_d
    }

    /// If the KeyE key is down(up), this function returns true(false).
    pub fn key_e(&self) -> bool {
        self.key_e
    }

    /// If the KeyF key is down(up), this function returns true(false).
    pub fn key_f(&self) -> bool {
        self.key_f
    }

    /// If the KeyG key is down(up), this function returns true(false).
    pub fn key_g(&self) -> bool {
        self.key_g
    }

    /// If the KeyH key is down(up), this function returns true(false).
    pub fn key_h(&self) -> bool {
        self.key_h
    }

    /// If the KeyI key is down(up), this function returns true(false).
    pub fn key_i(&self) -> bool {
        self.key_i
    }

    /// If the KeyJ key is down(up), this function returns true(false).
    pub fn key_j(&self) -> bool {
        self.key_j
    }

    /// If the KeyK key is down(up), this function returns true(false).
    pub fn key_k(&self) -> bool {
        self.key_k
    }

    /// If the KeyL key is down(up), this function returns true(false).
    pub fn key_l(&self) -> bool {
        self.key_l
    }

    /// If the KeyM key is down(up), this function returns true(false).
    pub fn key_m(&self) -> bool {
        self.key_m
    }

    /// If the KeyN key is down(up), this function returns true(false).
    pub fn key_n(&self) -> bool {
        self.key_n
    }

    /// If the KeyO key is down(up), this function returns true(false).
    pub fn key_o(&self) -> bool {
        self.key_o
    }

    /// If the KeyP key is down(up), this function returns true(false).
    pub fn key_p(&self) -> bool {
        self.key_p
    }

    /// If the KeyQ key is down(up), this function returns true(false).
    pub fn key_q(&self) -> bool {
        self.key_q
    }

    /// If the KeyR key is down(up), this function returns true(false).
    pub fn key_r(&self) -> bool {
        self.key_r
    }

    /// If the KeyS key is down(up), this function returns true(false).
    pub fn key_s(&self) -> bool {
        self.key_s
    }

    /// If the KeyT key is down(up), this function returns true(false).
    pub fn key_t(&self) -> bool {
        self.key_t
    }

    /// If the KeyU key is down(up), this function returns true(false).
    pub fn key_u(&self) -> bool {
        self.key_u
    }

    /// If the KeyV key is down(up), this function returns true(false).
    pub fn key_v(&self) -> bool {
        self.key_v
    }

    /// If the KeyW key is down(up), this function returns true(false).
    pub fn key_w(&self) -> bool {
        self.key_w
    }

    /// If the KeyX key is down(up), this function returns true(false).
    pub fn key_x(&self) -> bool {
        self.key_x
    }

    /// If the KeyY key is down(up), this function returns true(false).
    pub fn key_y(&self) -> bool {
        self.key_y
    }

    /// If the KeyZ key is down(up), this function returns true(false).
    pub fn key_z(&self) -> bool {
        self.key_z
    }
}

impl KeyEvent {
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
            web_sys::KeyEvent::DOM_VK_0 => {
                self.digit_0 = true;
            }
            web_sys::KeyEvent::DOM_VK_1 => {
                self.digit_1 = true;
            }
            web_sys::KeyEvent::DOM_VK_2 => {
                self.digit_2 = true;
            }
            web_sys::KeyEvent::DOM_VK_3 => {
                self.digit_3 = true;
            }
            web_sys::KeyEvent::DOM_VK_4 => {
                self.digit_4 = true;
            }
            web_sys::KeyEvent::DOM_VK_5 => {
                self.digit_5 = true;
            }
            web_sys::KeyEvent::DOM_VK_6 => {
                self.digit_6 = true;
            }
            web_sys::KeyEvent::DOM_VK_7 => {
                self.digit_7 = true;
            }
            web_sys::KeyEvent::DOM_VK_8 => {
                self.digit_8 = true;
            }
            web_sys::KeyEvent::DOM_VK_9 => {
                self.digit_9 = true;
            }
            web_sys::KeyEvent::DOM_VK_A => {
                self.key_a = true;
            }
            web_sys::KeyEvent::DOM_VK_B => {
                self.key_b = true;
            }
            web_sys::KeyEvent::DOM_VK_C => {
                self.key_c = true;
            }
            web_sys::KeyEvent::DOM_VK_D => {
                self.key_d = true;
            }
            web_sys::KeyEvent::DOM_VK_E => {
                self.key_e = true;
            }
            web_sys::KeyEvent::DOM_VK_F => {
                self.key_f = true;
            }
            web_sys::KeyEvent::DOM_VK_G => {
                self.key_g = true;
            }
            web_sys::KeyEvent::DOM_VK_H => {
                self.key_h = true;
            }
            web_sys::KeyEvent::DOM_VK_I => {
                self.key_i = true;
            }
            web_sys::KeyEvent::DOM_VK_J => {
                self.key_j = true;
            }
            web_sys::KeyEvent::DOM_VK_K => {
                self.key_k = true;
            }
            web_sys::KeyEvent::DOM_VK_L => {
                self.key_l = true;
            }
            web_sys::KeyEvent::DOM_VK_M => {
                self.key_m = true;
            }
            web_sys::KeyEvent::DOM_VK_N => {
                self.key_n = true;
            }
            web_sys::KeyEvent::DOM_VK_O => {
                self.key_o = true;
            }
            web_sys::KeyEvent::DOM_VK_P => {
                self.key_p = true;
            }
            web_sys::KeyEvent::DOM_VK_Q => {
                self.key_q = true;
            }
            web_sys::KeyEvent::DOM_VK_R => {
                self.key_r = true;
            }
            web_sys::KeyEvent::DOM_VK_S => {
                self.key_s = true;
            }
            web_sys::KeyEvent::DOM_VK_T => {
                self.key_t = true;
            }
            web_sys::KeyEvent::DOM_VK_U => {
                self.key_u = true;
            }
            web_sys::KeyEvent::DOM_VK_V => {
                self.key_v = true;
            }
            web_sys::KeyEvent::DOM_VK_W => {
                self.key_w = true;
            }
            web_sys::KeyEvent::DOM_VK_X => {
                self.key_x = true;
            }
            web_sys::KeyEvent::DOM_VK_Y => {
                self.key_y = true;
            }
            web_sys::KeyEvent::DOM_VK_Z => {
                self.key_z = true;
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
            web_sys::KeyEvent::DOM_VK_0 => {
                self.digit_0 = false;
            }
            web_sys::KeyEvent::DOM_VK_1 => {
                self.digit_1 = false;
            }
            web_sys::KeyEvent::DOM_VK_2 => {
                self.digit_2 = false;
            }
            web_sys::KeyEvent::DOM_VK_3 => {
                self.digit_3 = false;
            }
            web_sys::KeyEvent::DOM_VK_4 => {
                self.digit_4 = false;
            }
            web_sys::KeyEvent::DOM_VK_5 => {
                self.digit_5 = false;
            }
            web_sys::KeyEvent::DOM_VK_6 => {
                self.digit_6 = false;
            }
            web_sys::KeyEvent::DOM_VK_7 => {
                self.digit_7 = false;
            }
            web_sys::KeyEvent::DOM_VK_8 => {
                self.digit_8 = false;
            }
            web_sys::KeyEvent::DOM_VK_9 => {
                self.digit_9 = false;
            }
            web_sys::KeyEvent::DOM_VK_A => {
                self.key_a = false;
            }
            web_sys::KeyEvent::DOM_VK_B => {
                self.key_b = false;
            }
            web_sys::KeyEvent::DOM_VK_C => {
                self.key_c = false;
            }
            web_sys::KeyEvent::DOM_VK_D => {
                self.key_d = false;
            }
            web_sys::KeyEvent::DOM_VK_E => {
                self.key_e = false;
            }
            web_sys::KeyEvent::DOM_VK_F => {
                self.key_f = false;
            }
            web_sys::KeyEvent::DOM_VK_G => {
                self.key_g = false;
            }
            web_sys::KeyEvent::DOM_VK_H => {
                self.key_h = false;
            }
            web_sys::KeyEvent::DOM_VK_I => {
                self.key_i = false;
            }
            web_sys::KeyEvent::DOM_VK_J => {
                self.key_j = false;
            }
            web_sys::KeyEvent::DOM_VK_K => {
                self.key_k = false;
            }
            web_sys::KeyEvent::DOM_VK_L => {
                self.key_l = false;
            }
            web_sys::KeyEvent::DOM_VK_M => {
                self.key_m = false;
            }
            web_sys::KeyEvent::DOM_VK_N => {
                self.key_n = false;
            }
            web_sys::KeyEvent::DOM_VK_O => {
                self.key_o = false;
            }
            web_sys::KeyEvent::DOM_VK_P => {
                self.key_p = false;
            }
            web_sys::KeyEvent::DOM_VK_Q => {
                self.key_q = false;
            }
            web_sys::KeyEvent::DOM_VK_R => {
                self.key_r = false;
            }
            web_sys::KeyEvent::DOM_VK_S => {
                self.key_s = false;
            }
            web_sys::KeyEvent::DOM_VK_T => {
                self.key_t = false;
            }
            web_sys::KeyEvent::DOM_VK_U => {
                self.key_u = false;
            }
            web_sys::KeyEvent::DOM_VK_V => {
                self.key_v = false;
            }
            web_sys::KeyEvent::DOM_VK_W => {
                self.key_w = false;
            }
            web_sys::KeyEvent::DOM_VK_X => {
                self.key_x = false;
            }
            web_sys::KeyEvent::DOM_VK_Y => {
                self.key_y = false;
            }
            web_sys::KeyEvent::DOM_VK_Z => {
                self.key_z = false;
            }
            _ => {}
        }
    }

    fn add_event_listener_with_callback(type_: &str, listener: &js_sys::Function) {
        web_sys::window()
            .expect("No global window.")
            .document()
            .expect("The window should have document.")
            .add_event_listener_with_callback(type_, listener)
            .expect("Failed to call add_event_listener_with_callback.");
    }
}
