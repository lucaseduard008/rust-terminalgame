#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Point2d<T> {
  pub x: T,
  pub y: T
}

impl<T> Point2d<T> {
  pub fn new(x: T, y: T) -> Self {
    Self {x, y}    
  }
}