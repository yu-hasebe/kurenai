pub trait KeyEvent {
    fn enter(&self) -> bool;
    fn arrow_left(&self) -> bool;
    fn arrow_up(&self) -> bool;
    fn arrow_right(&self) -> bool;
    fn arrow_down(&self) -> bool;
}

pub struct HtmlKeyboardEvent {
    enter: bool,
    arrow_left: bool,
    arrow_up: bool,
    arrow_right: bool,
    arrow_down: bool,
}

impl KeyEvent for HtmlKeyboardEvent {
    fn enter(&self) -> bool {
        self.enter
    }

    fn arrow_left(&self) -> bool {
        self.arrow_left
    }

    fn arrow_up(&self) -> bool {
        self.arrow_up
    }

    fn arrow_right(&self) -> bool {
        self.arrow_right
    }

    fn arrow_down(&self) -> bool {
        self.arrow_down
    }
}
