use crate::game_error::GameError;
use std::{cell::RefCell, clone::Clone, collections::HashMap, ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
pub struct Image {
    image_id: ImageId,
    source_image: Rc<web_sys::HtmlImageElement>,
    begin_dot_x: i64,
    begin_dot_y: i64,
    width: i64,
    height: i64,
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
        begin_dot_x: i64,
        begin_dot_y: i64,
        width: i64,
        height: i64,
    ) -> Self {
        Self {
            image_id,
            source_image,
            begin_dot_x,
            begin_dot_y,
            width,
            height,
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

    pub fn begin_dot_x(&self) -> &i64 {
        &self.begin_dot_x
    }

    pub fn begin_dot_y(&self) -> &i64 {
        &self.begin_dot_y
    }

    pub fn width(&self) -> &i64 {
        &self.width
    }

    pub fn height(&self) -> &i64 {
        &self.height
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

    pub fn find(&self, image_id: &ImageId) -> Option<Image> {
        match self.store.borrow().get(image_id) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    pub fn save(&self, image: Image) {
        self.store.borrow_mut().insert(*image.image_id(), image);
    }
}
