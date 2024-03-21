use num::ToPrimitive;

use crate::{point::Point2d, traits::Position, unit::Player};

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

  fn normalize<T>(xy: (T, T)) -> (T, T)
where
    T: num::Float,
{
    let (x, y) = xy;
    let magnitude = (x * x + y * y).sqrt();
    (x / magnitude, y / magnitude)
}

  pub fn move_towards_player(&mut self, player: &Player) {
    let relative_direction = player.position()- self.position();
    let normalized_position = Self::normalize((relative_direction.x, relative_direction.y));
    let new_position = Point2d::new(normalized_position.0 * self.speed.to_f32().unwrap(), normalized_position.1 * self.speed.to_f32().unwrap());
    self.position += new_position;
  }
}

impl std::fmt::Display for Enemy {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "E")
  }
}
