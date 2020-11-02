use crate::game_error::GameError;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Canvas {
    id: CanvasId,
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(id: CanvasId, html_canvas_element_id: &str) -> Result<Self, GameError> {
        let canvas = Self::get_canvas_element_by_id(html_canvas_element_id);
        let context = Self::get_context(&canvas);
        Ok(Self { id, context })
    }
}

impl Canvas {
    pub fn id(&self) -> &CanvasId {
        &self.id
    }

    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }
}

impl Canvas {
    fn get_canvas_element_by_id(id: &str) -> web_sys::HtmlCanvasElement {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
    }

    fn get_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CanvasId(pub usize);

#[derive(Clone, Debug)]
pub struct CanvasRepository {
    store: Rc<RefCell<HashMap<CanvasId, Canvas>>>,
}

impl CanvasRepository {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn find(&self, canvas_id: &CanvasId) -> Option<Canvas> {
        match self.store.borrow().get(canvas_id) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    pub fn save(&self, canvas: Canvas) {
        self.store.borrow_mut().insert(*canvas.id(), canvas);
    }
}
