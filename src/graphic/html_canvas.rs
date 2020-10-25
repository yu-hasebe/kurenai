use crate::graphic::{dot::Dot, html_image::HtmlImage};
use wasm_bindgen::JsCast;

pub struct HtmlCanvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl HtmlCanvas {
    pub fn new(id: &str, size: &Dot) -> Self {
        let canvas = Self::create_html_canvas_element(id, size);
        // TODO: Add the canvas to an html element
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

    fn context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }
}
