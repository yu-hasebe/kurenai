pub trait Point<T>
where
    Self: Sized,
{
    fn new(x: i64, y: i64) -> Self;
    fn x(&self) -> &i64;
    fn y(&self) -> &i64;
}

#[derive(Clone, Copy, Debug)]
pub enum Dot {}
