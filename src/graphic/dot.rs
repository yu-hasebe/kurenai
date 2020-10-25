use derive_new::new;

#[derive(Clone, Copy, Debug, Eq, new, PartialEq)]
pub struct Dot {
    x: DotCoord,
    y: DotCoord,
}

impl Dot {
    pub fn x(&self) -> &DotCoord {
        &self.x
    }

    pub fn y(&self) -> &DotCoord {
        &self.y
    }
}

#[derive(Clone, Copy, Debug, Eq, new, PartialEq)]
pub struct DotCoord(pub i64);
