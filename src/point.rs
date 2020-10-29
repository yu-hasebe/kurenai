pub trait Point<T, U> {
    fn new(x: U, y: U) -> Self;
    fn x(&self) -> U;
    fn y(&self) -> U;
}

pub enum Dot {}
