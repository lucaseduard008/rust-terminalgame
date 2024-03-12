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
}
