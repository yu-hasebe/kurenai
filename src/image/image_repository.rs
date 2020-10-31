use crate::{
    image::{image_id::ImageId, Image},
    point::{Dot, Point},
};
use std::{clone::Clone, rc::Rc};

#[derive(Clone, Debug)]
pub struct ImageRepository<T>
where
    T: Clone + Point<Dot>,
{
    image: Image<T>,
}

impl<T> ImageRepository<T>
where
    T: Clone + Point<Dot>,
{
    pub fn new(
        image_id: ImageId,
        source_image: Rc<web_sys::HtmlImageElement>,
        begin_dot_on_source_image: T,
        size: T,
    ) -> Self {
        Self {
            image: Image::<T>::new(image_id, source_image, begin_dot_on_source_image, size),
        }
    }
    pub fn find(&self, image_id: ImageId) -> Result<Image<T>, String> {
        Ok(self.image.clone())
    }
    pub fn save(&self) -> Result<(), String> {
        Ok(())
    }
}
