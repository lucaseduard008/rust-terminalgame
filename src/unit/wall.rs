use num::ToPrimitive;

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

  pub fn draw(&self, buffer: &mut Vec<u8>) {
    let position = self.position();

    crossterm::queue!(
        buffer,
        crossterm::cursor::MoveTo(
            position
                .x
                .to_f64()
                .expect("could not convert position x to f64")
                .round() as u16
                + 1,
            position
                .y
                .to_f64()
                .expect("could not convert position y to f64")
                .round() as u16
                + 1,
        ),
        crossterm::style::Print(self)
    )
    .unwrap();
}
}

impl std::fmt::Display for Wall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "#")
  }
}