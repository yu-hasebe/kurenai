pub trait Point<T, U> {
    fn new(x: U, y: U) -> Self;
    fn x(&self) -> &U;
    fn y(&self) -> &U;
}

#[derive(Clone, Copy, Debug)]
pub enum Dot {}
