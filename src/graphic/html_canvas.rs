use crate::graphic::{dot::Dot, html_image::HtmlImage};
use wasm_bindgen::JsCast;

pub struct HtmlCanvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl HtmlCanvas {
    pub fn new(canvas_id: &str, canvas_size: &Dot, id_append_to: &str) -> Self {
        let canvas = Self::create_html_canvas_element(canvas_id, canvas_size);
        Self::append_html_canvas_element_to(id_append_to, &canvas);
        let context = Self::context(&canvas);
        Self { context }
    }

    pub fn draw(
        &self,
        image: &HtmlImage,
        begin_at_on_image: &Dot,
        size_on_image: &Dot,
        begin_at_on_canvas: &Dot,
        size_on_canvas: &Dot,
    ) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image.image(),
                begin_at_on_image.x().0 as f64,
                begin_at_on_image.y().0 as f64,
                size_on_image.x().0 as f64,
                size_on_image.y().0 as f64,
                begin_at_on_canvas.x().0 as f64,
                begin_at_on_canvas.y().0 as f64,
                size_on_canvas.x().0 as f64,
                size_on_canvas.y().0 as f64,
            )
            .unwrap();
    }

    fn create_html_canvas_element(id: &str, size: &Dot) -> web_sys::HtmlCanvasElement {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap();
        canvas.set_attribute("id", id).unwrap();
        canvas
            .set_attribute("width", size.x().0.to_string().as_str())
            .unwrap();
        canvas
            .set_attribute("height", size.y().0.to_string().as_str())
            .unwrap();
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
