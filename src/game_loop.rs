use crate::{
    canvas::Canvas, game_error::GameError, image::ImageRepository, key_event::KeyEvent,
    traits::game_service::GameService,
};
use std::{cell::RefCell, clone::Clone, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Debug)]
pub struct GameLoop;

impl GameLoop {
    pub fn run<T: 'static>(
        game_service: T,
        image_repository: ImageRepository,
        canvas: Canvas,
    ) -> Result<(), GameError>
    where
        T: GameService,
    {
        let game_service_rc = Rc::new(game_service);
        let image_repository_rc = Rc::new(image_repository);
        let canvas_rc = Rc::new(canvas);
        let key_event_rc = Rc::new(RefCell::new(KeyEvent::new()));
        KeyEvent::run(key_event_rc.clone())?;

        let closure_rc = Rc::new(RefCell::new(None));
        let closure = closure_rc.clone();
        *closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let game_service = game_service_rc.clone();
            let key_event = key_event_rc.borrow();
            let image_repository = image_repository_rc.clone();
            let canvas = canvas_rc.clone();
            game_service.key_event(&key_event);
            game_service.update();
            game_service.draw(&image_repository, &canvas);
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
