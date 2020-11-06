//! This crate is a 2D game engine based on wasm and HTMLCanvas. It enables you to create a 2D game
//! easily with your domain definition.
//!
//! A minimum implementation is the following:
//! ```
//! use kurenai::game_loop;
//! use kurenai::game_service::GameService;
//! use kurenai::key_event::KeyEvent;
//! use kurenai::{canvas, image};
//!
//! use std::cell::RefCell;
//! use std::ops::Deref;
//! use std::rc::Rc;
//! use wasm_bindgen_test::*;
//!
//! // Wrap mutable data with RefCell.
//! struct TestGameService {
//!     data: RefCell<i64>,
//!     image: Rc<web_sys::HtmlImageElement>,
//! }
//!
//! // Implement the following three functions.
//! impl GameService for TestGameService {
//!     fn key_event(&self, key_event: &KeyEvent) {
//!         if key_event.enter() {
//!             let mut data = self.data.borrow_mut();
//!             *data = 0;
//!         }
//!     }
//!
//!     fn update(&self) {
//!         let mut data = self.data.borrow_mut();
//!         *data += 1;
//!     }
//!
//!     fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
//!         let image = self.image();
//!         // You can draw various images with CanvasRenderingContext2D. You can find the APIs at
//!         // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html.
//!         context
//!             .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
//!                 image,
//!                 0.0,
//!                 0.0,
//!                 32.0,
//!                 32.0,
//!                 self.data.borrow().clone() as f64,
//!                 self.data.borrow().clone() as f64,
//!                 32.0,
//!                 32.0,
//!             )
//!             .expect(format!("Failed to draw image {:?}", image).as_str());
//!     }
//! }
//!
//! impl TestGameService {
//!     fn new() -> Self {
//!         let image = image::create_new_html_image_element(&[], "gif");
//!         Self {
//!             data: RefCell::new(0),
//!             image: Rc::new(image),
//!         }
//!     }
//!
//!     fn image(&self) -> &web_sys::HtmlImageElement {
//!         self.image.deref()
//!     }
//! }
//!
//! #[wasm_bindgen_test]
//! fn pass() {
//!     // Pass the GameService implementation and CanvasRenderingContext2D to the game_loop::run()
//!     // function.
//!     let test_game_service = TestGameService::new();
//!     let canvas_rendering_context = canvas::get_canvas_rendering_context_2d("main-canvas");
//!     game_loop::run(test_game_service, canvas_rendering_context);
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(bare_trait_objects)]
#![deny(missing_copy_implementations)]

/// This module has only a game_loop::run() function. Pass the GameService implementation and a
/// CanvasRenderingContext2D to the function. Then you can start the game loop.
pub mod game_loop;

/// This module has only a GameService trait. Create a struct that implements the trait. The
/// struct should have data and images.
pub mod game_service;

/// This module enables you to create a CanvasRenderingContext2D, from which you can draw images
/// on the canvas. Pass it to game_loop::run().
pub mod canvas;

/// This module enables you to create HTMLImageElements. Add them to the GameService
/// implementation.
pub mod image;

/// This module has KeyEvent struct, from which you can know which key is down and which is up.
/// You can use it at the GameService::key_event() implementation.
pub mod key_event;
