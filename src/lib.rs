//! You can find how to use this crate at `/kurenai/tests/wasm.rs`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(bare_trait_objects)]
#![deny(missing_copy_implementations)]

/// This module has only a game_loop::run() function. Pass the GameService implementation and a
/// CanvasRenderingContext2D to the function.
pub mod game_loop;

/// This module has only a trait GameService. Createa a struct that implements the trait. The
/// struct should have data(and/or repository) and images.
pub mod game_service;

/// This module enables you to create a CanvasRenderingContext2D, from which you can draw images
/// on the canvas. Pass it to game_loop::run().
pub mod canvas;

/// This module enables you to create HTMLImageElements. Pass them to the GameService
/// implementation.
pub mod image;

/// This module shows KeyEvent struct, from which you can know which key is down and which is up.
/// You can use it at the GameService::key_event() implementation.
pub mod key_event;
