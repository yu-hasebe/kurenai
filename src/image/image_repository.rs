use crate::{
    game_error::GameError,
    image::{image_id::ImageId, Image},
    point::{Dot, Point},
};
use std::{cell::RefCell, clone::Clone, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct ImageRepository<T>
where
    T: Clone + Point<Dot>,
{
    store: Rc<RefCell<HashMap<ImageId, Image<T>>>>,
}

impl<T> ImageRepository<T>
where
    T: Clone + Point<Dot>,
{
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    pub fn find(&self, image_id: &ImageId) -> Result<Image<T>, GameError> {
        match self.store.borrow().get(image_id) {
            Some(r) => Ok(r.clone()),
            None => Err(GameError::RepositoryError(format!(
                "Image with ImageId {:?} is not found in this repository.",
                *image_id
            ))),
        }
    }
    pub fn save(&self, image: Image<T>) -> Result<(), GameError> {
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
