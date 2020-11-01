#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dot {
    x: i64,
    y: i64,
}

impl Dot {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &i64 {
        &self.x
    }

    pub fn y(&self) -> &i64 {
        &self.y
    }
}
