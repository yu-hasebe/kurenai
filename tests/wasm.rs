use kurenai::canvas;
use kurenai::game_loop::GameLoop;
use kurenai::game_service::GameService;
use kurenai::key_event::KeyEvent;

use wasm_bindgen_test::*;

struct TestGameService {}

impl GameService for TestGameService {
    fn key_event(&self, key_event: &KeyEvent) {
        if key_event.enter() {
            // Update data
        }
    }
    fn update(&self) {
        // Update data
    }
    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        // Draw
    }
}

impl TestGameService {
    fn new() -> Self {
        Self {}
    }
}

#[wasm_bindgen_test]
fn pass() {
    let test_game_service = TestGameService::new();
    let canvas_rendering_context = canvas::get_canvas_rendering_context_2d("main-canvas");
    GameLoop::run(test_game_service, canvas_rendering_context);
}
