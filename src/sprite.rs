use crate::{
    canvas::Canvas,
    image::{image_id::ImageId, image_repository::ImageRepository},
    point::{Dot, Point},
};
use num_traits::{NumAssign, ToPrimitive};

/// You can draw structs that implement Sprite.
pub trait Sprite<T, U>
where
    T: Point<Dot, U>,
    U: NumAssign + ToPrimitive,
{
    fn draw(
        &self,
        image_repository: &ImageRepository<T, U>,
        canvas: &Canvas,
        begin_dot_on_canvas: T,
    ) -> Result<(), String> {
        let image = image_repository.find(*self.image_id());
        canvas.draw_image_with_html_image_element(
            image.source_image(),
            *image.begin_dot_on_source_image(),
            *image.size(),
            begin_dot_on_canvas,
            *self.size(),
        )
    }

    fn image_id(&self) -> &ImageId;
    fn size(&self) -> &T;
}
