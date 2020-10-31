use crate::{
    canvas::Canvas,
    image::{image_id::ImageId, image_repository::ImageRepository},
    point::{Dot, Point},
};
use std::{clone::Clone, ops::Deref};

/// You can draw structs that implement Sprite.
pub trait Sprite<T>
where
    T: Clone + Point<Dot>,
{
    fn draw(
        &self,
        image_repository: &ImageRepository<T>,
        canvas: &Canvas,
        begin_dot_on_canvas: T,
    ) -> Result<(), String> {
        let image = image_repository.find(self.image_id()).unwrap();
        canvas.draw_image_with_html_image_element(
            image.source_image().deref(),
            image.begin_dot_on_source_image().clone(),
            image.size().clone(),
            begin_dot_on_canvas,
            self.size().clone(),
        )
    }

    fn image_id(&self) -> &ImageId;
    fn size(&self) -> &T;
}
