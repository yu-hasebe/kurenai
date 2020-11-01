use crate::{
    canvas::Canvas,
    game_error::GameError,
    game_state::GameState,
    image::image_repository::ImageRepository,
    key_event::KeyEvent,
    point::{Dot, Point},
};
use std::{cell::RefCell, clone::Clone, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Debug)]
pub struct GameLoop;

impl GameLoop {
    pub fn run<T: 'static, U: 'static, V: 'static>(
        game_state: T,
        image_repository: ImageRepository<V>,
        canvas: Canvas,
    ) -> Result<(), GameError>
    where
        T: GameState<U, V>,
        U: KeyEvent,
        V: Clone + Point<Dot>,
    {
        let game_state_rc = Rc::new(RefCell::new(game_state));
        let image_repository_rc = Rc::new(image_repository);
        let canvas_rc = Rc::new(canvas);
        let key_event_rc = Rc::new(RefCell::new(U::new()));
        U::run(key_event_rc.clone())?;

        let closure_rc = Rc::new(RefCell::new(None));
        let closure = closure_rc.clone();
        *closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut game_state = game_state_rc.borrow_mut();
            let key_event = key_event_rc.borrow();
            let image_repository = image_repository_rc.clone();
            let canvas = canvas_rc.clone();
            game_state.key_event(&key_event);
            game_state.update();
            game_state.draw(&image_repository, &canvas);
            Self::request_animation_frame(closure_rc.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        Self::request_animation_frame(closure.borrow().as_ref().unwrap());
        Ok(())
    }
}

impl GameLoop {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }
}
