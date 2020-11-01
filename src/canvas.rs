use crate::{
    game_error::GameError,
    point::{Dot, Point},
};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Canvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new<T>(canvas_id: &str, canvas_size: T, id_append_to: &str) -> Result<Self, GameError>
    where
        T: Point<Dot>,
    {
        let canvas = Self::create_html_canvas_element(
            canvas_id,
            canvas_size.x().to_string().as_str(),
            canvas_size.y().to_string().as_str(),
        );
        Self::append_html_canvas_element_to(id_append_to, &canvas);
        let context = Self::context(&canvas);
        Ok(Self { context })
    }

    pub fn draw_image_with_html_image_element<T>(
        &self,
        image: &web_sys::HtmlImageElement,
        begin_dot_on_image: T,
        size_dot_on_image: T,
        begin_dot_on_canvas: T,
        size_dot_on_canvas: T,
    ) -> Result<(), GameError>
    where
        T: Point<Dot>,
    {
        let begin_dot_x_on_image = *begin_dot_on_image.x() as f64;
        let begin_dot_y_on_image = *begin_dot_on_image.y() as f64;
        let size_dot_x_on_image = *size_dot_on_image.x() as f64;
        let size_dot_y_on_image = *size_dot_on_image.y() as f64;
        let begin_dot_x_on_canvas = *begin_dot_on_canvas.x() as f64;
        let begin_dot_y_on_canvas = *begin_dot_on_canvas.y() as f64;
        let size_dot_x_on_canvas = *size_dot_on_canvas.x() as f64;
        let size_dot_y_on_canvas = *size_dot_on_canvas.y() as f64;
        match self
            .context
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
            ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
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
