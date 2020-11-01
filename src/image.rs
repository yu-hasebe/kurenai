use crate::{dot::Dot, game_error::GameError};
use std::{cell::RefCell, clone::Clone, collections::HashMap, ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
pub struct Image {
    image_id: ImageId,
    source_image: Rc<web_sys::HtmlImageElement>,
    begin_dot: Dot,
    size: Dot,
}

impl Image {
    pub fn create_new_html_image_element(
        bytes: &[u8],
        extension: &str,
    ) -> Result<web_sys::HtmlImageElement, GameError> {
        // TODO: You need to know its extension from bytes.
        let image = web_sys::HtmlImageElement::new().unwrap();
        let src = format!(
            "data:image/{};base64,{}",
            extension,
            base64::encode(&bytes.to_vec())
        );
        image.set_src(&src);
        Ok(image)
    }

    pub fn new(
        image_id: ImageId,
        source_image: Rc<web_sys::HtmlImageElement>,
        begin_dot: Dot,
        size: Dot,
    ) -> Self {
        Self {
            image_id,
            source_image,
            begin_dot,
            size,
        }
    }
}

impl Image {
    pub fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    pub fn source_image(&self) -> &web_sys::HtmlImageElement {
        self.source_image.deref()
    }

    pub fn begin_dot(&self) -> &Dot {
        &self.begin_dot
    }

    pub fn size(&self) -> &Dot {
        &self.size
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ImageId(pub usize);

#[derive(Clone, Debug)]
pub struct ImageRepository {
    store: Rc<RefCell<HashMap<ImageId, Image>>>,
}

impl ImageRepository {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn find(&self, image_id: &ImageId) -> Result<Image, GameError> {
        match self.store.borrow().get(image_id) {
            Some(r) => Ok(r.clone()),
            None => Err(GameError::RepositoryError(format!(
                "Image with ImageId {:?} is not found in this repository.",
                *image_id
            ))),
        }
    }
    pub fn save(&self, image: Image) -> Result<(), GameError> {
        let image_id = *image.image_id();
        match self.store.borrow_mut().insert(image_id, image) {
            Some(_) => Err(GameError::RepositoryError(format!(
                "Image with ImageId {:?} is already saved.",
                image_id
            ))),
            None => Ok(()),
        }
    }
}
