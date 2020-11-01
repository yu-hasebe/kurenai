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
    pub fn new(
        id: CanvasId,
        canvas_id: &str,
        canvas_width: i64,
        canvas_height: i64,
        id_append_to: &str,
    ) -> Result<Self, GameError> {
        let canvas = Self::create_html_canvas_element(
            canvas_id,
            canvas_width.to_string().as_str(),
            canvas_height.to_string().as_str(),
        );
        Self::append_html_canvas_element_to(id_append_to, &canvas);
        let context = Self::context_(&canvas);
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
    fn create_html_canvas_element(
        id: &str,
        width: &str,
        height: &str,
    ) -> web_sys::HtmlCanvasElement {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap();
        canvas.set_attribute("id", id).unwrap();
        canvas.set_attribute("width", width).unwrap();
        canvas.set_attribute("height", height).unwrap();
        canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap()
    }

    fn append_html_canvas_element_to(id: &str, canvas: &web_sys::HtmlCanvasElement) {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)
            .unwrap()
            .append_child(canvas)
            .unwrap();
    }

    fn context_(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
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
