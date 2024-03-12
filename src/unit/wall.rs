use crate::point::Point2d;
use crate::traits::Position;

#[derive(Default)]
pub struct Wall {
  position: Point2d<u16>
}

impl Position<u16> for Wall {
  fn position(&self) -> Point2d<u16> {
      self.position
  }

  fn set_position(&mut self, position: Point2d<u16>) {
      self.position = position;
  }
}

impl Wall {
  pub fn new(x: u16, y: u16) -> Self {
    Self{
      position: Point2d::new(x, y)
    }
  }
}