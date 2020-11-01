use crate::{
    canvas::Canvas,
    game_error::GameError,
    image::{image_id::ImageId, image_repository::ImageRepository},
    point::{Dot, Point},
};
use std::clone::Clone;

pub trait Sprite<T>
where
    T: Clone + Point<Dot>,
{
    fn draw(
        &self,
        image_repository: &ImageRepository<T>,
        canvas: &Canvas,
        begin_dot_on_canvas: T,
    ) -> Result<(), GameError> {
        let image = image_repository.find(self.image_id())?;
        canvas
            .draw_image_with_html_image_element(
                image.source_image(),
                image.begin_dot_on_source_image().clone(),
                image.size().clone(),
                begin_dot_on_canvas,
                self.size().clone(),
            )
            .unwrap();
        Ok(())
    }

    fn image_id(&self) -> &ImageId;
    fn size(&self) -> &T;
}
