use crate::point::{Dot, Point};
use num_traits::{NumAssign, ToPrimitive};
use std::string::ToString;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Canvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    /// Create a new Canvas instance and append it to an html_element.
    pub fn new<T, U>(canvas_id: &str, canvas_size: T, id_append_to: &str) -> Self
    where
        T: Point<Dot, U>,
        U: NumAssign + ToPrimitive + ToString,
    {
        let canvas = Self::create_html_canvas_element(
            canvas_id,
            canvas_size.x().to_string().as_str(),
            canvas_size.y().to_string().as_str(),
        );
        Self::append_html_canvas_element_to(id_append_to, &canvas);
        let context = Self::context(&canvas);
        Self { context }
    }

    /// Call this function through Sprite trait. Do not call it directly.
    pub fn draw_image_with_html_image_element<T, U>(
        &self,
        image: &web_sys::HtmlImageElement,
        begin_dot_on_image: T,
        size_dot_on_image: T,
        begin_dot_on_canvas: T,
        size_dot_on_canvas: T,
    ) -> Result<(), String>
    where
        T: Point<Dot, U>,
        U: NumAssign + ToPrimitive,
    {
        let begin_dot_x_on_image = begin_dot_on_image.x().to_f64().ok_or("parse error")?;
        let begin_dot_y_on_image = begin_dot_on_image.y().to_f64().ok_or("parse error")?;
        let size_dot_x_on_image = size_dot_on_image.x().to_f64().ok_or("parse error")?;
        let size_dot_y_on_image = size_dot_on_image.y().to_f64().ok_or("parse error")?;
        let begin_dot_x_on_canvas = begin_dot_on_canvas.x().to_f64().ok_or("parse error")?;
        let begin_dot_y_on_canvas = begin_dot_on_canvas.y().to_f64().ok_or("parse error")?;
        let size_dot_x_on_canvas = size_dot_on_canvas.x().to_f64().ok_or("parse error")?;
        let size_dot_y_on_canvas = size_dot_on_canvas.y().to_f64().ok_or("parse error")?;
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                begin_dot_x_on_image,
                begin_dot_y_on_image,
                size_dot_x_on_image,
                size_dot_y_on_image,
                begin_dot_x_on_canvas,
                begin_dot_y_on_canvas,
                size_dot_x_on_canvas,
                size_dot_y_on_canvas,
            )
            .unwrap();
        // TODO: Convert wasm_bindgen::JsValue to String in Result
        Ok(())
    }

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
