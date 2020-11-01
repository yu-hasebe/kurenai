use crate::{
    canvas::Canvas,
    game_loop::GameLoop,
    image::{Image, ImageId, ImageRepository},
    key_event::KeyEvent,
    traits::game_service::GameService,
};
use std::rc::Rc;

struct TestGameService {}

impl GameService for TestGameService {
    fn key_event(&self, key_event: &KeyEvent) {
        if key_event.enter() {
            // Update data
        }
    }
    fn update(&self) {
        // Update data
    }
    fn draw(&self, image_repository: &ImageRepository, canvas: &Canvas) {
        // Draw
    }
}

impl TestGameService {
    fn new() -> Self {
        Self {}
    }
}

#[test]
#[should_panic]
fn main() {
    let test_game_service = TestGameService::new();
    let canvas = Canvas::new("main-canvas", 480, 480, "game-container").unwrap();

    // image_repository factory
    let image_repository = {
        let new_html_image_element_rc =
            Rc::new(Image::create_new_html_image_element(&[], "gif").unwrap());
        let image_repository = ImageRepository::new();
        image_repository.save(Image::new(
            ImageId(0),
            new_html_image_element_rc.clone(),
            64,
            32,
            32,
            32,
        ));
        image_repository.save(Image::new(
            ImageId(1),
            new_html_image_element_rc.clone(),
            64,
            64,
            32,
            32,
        ));
        image_repository
    };

    GameLoop::run(test_game_service, image_repository, canvas).unwrap();
}
