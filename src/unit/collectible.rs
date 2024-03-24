use num::ToPrimitive;

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

impl std::fmt::Display for Collectible {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", crossterm::style::Stylize::red("♥"))
  }
}

impl Collectible {
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