pub mod image_id;
pub mod image_repository;
use crate::point::{Dot, Point};
use image_id::ImageId;
use num_traits::{NumAssign, ToPrimitive};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Image<T, U>
where
    T: Point<Dot, U>,
    U: NumAssign + ToPrimitive,
{
    image_id: ImageId,
    source_image: Rc<web_sys::HtmlImageElement>,
    begin_dot_on_source_image: T,
    size: T,
}

impl<T, U> Image<T, U>
where
    T: Point<Dot, U>,
    U: NumAssign + ToPrimitive,
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

    pub fn source_image(&self) -> &web_sys::HtmlImageElement {
        &self.source_image.clone()
    }

    pub fn begin_dot_on_source_image(&self) -> &T {
        &self.begin_dot_on_source_image
    }

    pub fn size(&self) -> &T {
        &self.size
    }
}

// TODO: You need to know its extension from bytes.
pub fn create_new_html_image_element(
    bytes: &[u8],
    extension: &str,
) -> Result<web_sys::HtmlImageElement, String> {
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
