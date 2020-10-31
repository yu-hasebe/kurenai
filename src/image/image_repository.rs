use crate::{
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
    pub fn find(&self, image_id: &ImageId) -> Result<Image<T>, String> {
        match self.store.borrow().get(image_id) {
            Some(r) => Ok(r.clone()),
            None => Err("Not found in ImageRepository".to_string()),
        }
    }
    pub fn save(&self, image: Image<T>) -> Result<(), String> {
        match self.store.borrow_mut().insert(*image.image_id(), image) {
            Some(_) => Err("Already inserted in ImageRepository".to_string()),
            None => Ok(()),
        }
    }
}
