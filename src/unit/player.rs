use std::fmt::write;

use num::ToPrimitive;

use crate::{point::Point2d, traits::Position};

#[derive(Default)]
struct CollisionStruct {
  x: u32,
  y: u32
}

#[derive(Default)]
pub struct Player {
  position: Point2d<f32>,
  direction: CollisionStruct,
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
      position: self.position, speed: self.speed, health: self.health, direction: self.direction
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

  pub fn accelerate(&mut self) {
    if self.speed < 1.0 {
      self.speed += 0.05;
    }
  }

  pub fn decelerate(&mut self) {
    if self.speed > 0.0 {
      self.speed -= 0.05;
    }
  }

  pub fn move_forward(&mut self) {
    todo!()
  }
  
  pub fn turn_left(&mut self) {
    todo!()
  }

  pub fn turn_right(&mut self) {
    todo!()
  }

  pub fn forward_position(&self) -> Point2d<f32> {
    todo!()
  }
}

impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "P")
  }
}

#[derive(Default)]
pub struct PlayerBuilder {
  position: Point2d<f32>,
  direction: CollisionStruct,
  speed: f64,
  health: u8
}

impl PlayerBuilder {
  pub fn new() -> Self {
    Self {
      position: Point2d::new(0.0, 0.0),
      direction: CollisionStruct{0, 0},
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
      health: self.health,
      direction: self.direction
    }
  }

  pub fn direction(self, x: f64, y: f64) {
    todo!()  
  }
}