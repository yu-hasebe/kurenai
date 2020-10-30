use crate::{
    canvas::Canvas,
    point::{Dot, Point},
};
use num_traits::{NumAssign, ToPrimitive};
use std::string::ToString;
use web_sys::HtmlImageElement;

/// You can draw structs that implement Sprite.
pub trait Sprite<T, U>
where
    T: Point<Dot, U>,
    U: NumAssign + ToPrimitive + ToString,
{
    fn draw(&self, canvas: &Canvas, begin_dot_on_canvas: T) -> Result<(), String> {
        canvas.draw_image_with_html_image_element(
            self.image(),
            self.begin_dot_on_image(),
            self.size(),
            begin_dot_on_canvas,
            self.size(),
        )
    }

    fn image(&self) -> &HtmlImageElement;
    fn begin_dot_on_image(&self) -> T;
    fn size(&self) -> T;
}
