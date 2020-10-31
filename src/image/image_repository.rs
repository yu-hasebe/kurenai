use crate::{
    image::{image_id::ImageId, Image},
    point::{Dot, Point},
};
use num_traits::{NumAssign, ToPrimitive};
use std::rc::Rc;

pub struct ImageRepository<T, U>
where
    T: Point<Dot, U>,
    U: NumAssign + ToPrimitive,
{
    image: Image<T, U>,
}

impl<T, U> ImageRepository<T, U>
where
    T: Point<Dot, U>,
    U: NumAssign + ToPrimitive,
{
    fn new(
        image_id: ImageId,
        source_image: Rc<web_sys::HtmlImageElement>,
        begin_dot_on_source_image: T,
        size: T,
    ) -> Self {
        Self {
            image: Image::<T, U>::new(image_id, source_image, begin_dot_on_source_image, size),
        }
    }
    fn find(&self) -> Result<Image<T, U>, String> {
        Err("error")
    }
    fn save(&self) -> Result<(), String> {
        Ok(())
    }
}
