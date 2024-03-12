use crate::point::Point2d;
use crate::traits::Position;

#[derive(Default)]
pub struct Collectible {
  pub position: Point2d<u16>
}

impl Position<u16> for Collectible {
  fn position(&self) -> Point2d<u16> {
      self.position
  }

  fn set_position(&mut self, position: Point2d<u16>) {
      self.position = position;
  }
}