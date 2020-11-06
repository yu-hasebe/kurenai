use kurenai::game_loop;
use kurenai::game_service::GameService;
use kurenai::key_event::KeyEvent;
use kurenai::{canvas, image};

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen_test::*;

struct TestGameService {
    data: RefCell<i64>,
    image: Rc<web_sys::HtmlImageElement>,
}

impl GameService for TestGameService {
    fn key_event(&self, key_event: &KeyEvent) {
        if key_event.enter() {
            let mut data = self.data.borrow_mut();
            *data = 0;
        }
    }

    fn update(&self) {
        let mut data = self.data.borrow_mut();
        *data += 1;
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        let image = self.image();
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                0.0,
                0.0,
                32.0,
                32.0,
                self.data.borrow().clone() as f64,
                self.data.borrow().clone() as f64,
                32.0,
                32.0,
            )
            .expect(format!("Failed to draw image {:?}", image).as_str());
    }
}

impl TestGameService {
    fn new() -> Self {
        let image = image::create_new_html_image_element(&[], "gif");
        Self {
            data: RefCell::new(0),
            image: Rc::new(image),
        }
    }

    fn image(&self) -> &web_sys::HtmlImageElement {
        self.image.deref()
    }
}

#[wasm_bindgen_test]
fn pass() {
    let test_game_service = TestGameService::new();
    let canvas_rendering_context = canvas::get_canvas_rendering_context_2d("main-canvas");
    game_loop::run(test_game_service, canvas_rendering_context);
}
