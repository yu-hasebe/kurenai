use crate::image::HtmlImage;
use wasm_bindgen::JsCast;

pub struct HtmlCanvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl HtmlCanvas {
    pub fn new(canvas_id: &str, canvas_width: i64, canvas_height: i64, id_append_to: &str) -> Self {
        let canvas = Self::create_html_canvas_element(canvas_id, canvas_width, canvas_height);
        Self::append_html_canvas_element_to(id_append_to, &canvas);
        let context = Self::context(&canvas);
        Self { context }
    }

    pub fn draw(
        &self,
        image: &HtmlImage,
        begin_x_on_image: i64,
        begin_y_on_image: i64,
        width_on_image: i64,
        height_on_image: i64,
        begin_x_on_canvas: i64,
        begin_y_on_canvas: i64,
        width_on_canvas: i64,
        height_on_canvas: i64,
    ) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image.image(),
                begin_x_on_image as f64,
                begin_y_on_image as f64,
                width_on_image as f64,
                height_on_image as f64,
                begin_x_on_canvas as f64,
                begin_y_on_canvas as f64,
                width_on_canvas as f64,
                height_on_canvas as f64,
            )
            .unwrap();
    }

    fn create_html_canvas_element(id: &str, width: i64, height: i64) -> web_sys::HtmlCanvasElement {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap();
        canvas.set_attribute("id", id).unwrap();
        canvas
            .set_attribute("width", width.to_string().as_str())
            .unwrap();
        canvas
            .set_attribute("height", height.to_string().as_str())
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
