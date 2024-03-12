use crate::{point::Point2d, traits::Position};

#[derive(Default)]
pub struct Player {
  position: Point2d<f32>,
  speed: f64,
  health: u8
}

impl Position<f32> for Player {
  fn position(&self) -> Point2d<f32> {
      self.position
  }

  fn set_position(&mut self, position: Point2d<f32>) {
      self.position = position;
  }
}

impl Player {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn build(&self) -> PlayerBuilder {
    PlayerBuilder {
      position: self.position, speed: self.speed, health: self.health
    }
  }

  pub fn is_alive(&self) -> bool {
    if self.health > 0 {
      return true;
    }
    false
  }

  pub fn take_damage(&mut self, damage: u8) {
    self.health -= damage;
  } 

  pub fn health(&self) -> u8 {
    self.health
  }

  pub fn speed(&self) -> f64 {
    self.speed
  }
}

#[derive(Default)]
pub struct PlayerBuilder {
  position: Point2d<f32>,
  speed: f64,
  health: u8
}

impl PlayerBuilder {
  pub fn new() -> Self {
    Self {
      position: Point2d::new(0.0, 0.0),
      speed: 0f64,
      health: 0
    }
  }

  pub fn speed(mut self, speed: f64) -> Self {
    self.speed = speed;
    self
  }

  pub fn health(mut self, health: u8) -> Self {
    self.health = health;
    self
  }

  pub fn build(self) -> Player {
    Player {
      position: self.position,
      speed: self.speed,
      health: self.health
    }
  }
}