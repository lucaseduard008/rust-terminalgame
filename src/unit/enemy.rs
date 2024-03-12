use num::ToPrimitive;

use crate::{point::Point2d, traits::Position};

#[derive(Default)]
pub struct Enemy {
  position: Point2d<f32>,
  speed: f64
}

impl Position<f32> for Enemy {
  fn position(&self) -> Point2d<f32> {
      self.position
  }

  fn set_position(&mut self, position: Point2d<f32>) {
      self.position = position;
  }
}

impl Enemy {
  pub fn with_speed(speed: f64) -> Self {
    Self {position: Point2d::new(0.0, 0.0), speed}
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

impl std::fmt::Display for Enemy {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "E")
  }
}
