use crate::game_error::GameError;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Canvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(
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
        let context = Self::context(&canvas);
        Ok(Self { context })
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

    fn context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }
}
