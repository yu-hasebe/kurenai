use crate::graphic::{dot::Dot, html_image::HtmlImage};

pub struct HtmlCanvas {
    context: web_sys::CanvasRenderingContext2d,
}

impl HtmlCanvas {
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
}
