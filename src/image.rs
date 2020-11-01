pub mod image_id;
pub mod image_repository;
use crate::{
    game_error::GameError,
    point::{Dot, Point},
};
use image_id::ImageId;
use std::{clone::Clone, ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
pub struct Image<T>
where
    T: Clone + Point<Dot>,
{
    image_id: ImageId,
    source_image: Rc<web_sys::HtmlImageElement>,
    begin_dot_on_source_image: T,
    size: T,
}

impl<T> Image<T>
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
            image_id,
            source_image,
            begin_dot_on_source_image,
            size,
        }
    }

    pub fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    pub fn source_image(&self) -> &web_sys::HtmlImageElement {
        self.source_image.deref()
    }

    pub fn begin_dot_on_source_image(&self) -> &T {
        &self.begin_dot_on_source_image
    }

    pub fn size(&self) -> &T {
        &self.size
    }

    // TODO: You need to know its extension from bytes.
    pub fn create_new_html_image_element(
        bytes: &[u8],
        extension: &str,
    ) -> Result<web_sys::HtmlImageElement, GameError> {
        // TODO: Add validation
        let image = web_sys::HtmlImageElement::new().unwrap();
        let src = format!(
            "data:image/{};base64,{}",
            extension,
            base64::encode(&bytes.to_vec())
        );
        image.set_src(&src);
        Ok(image)
    }
}
