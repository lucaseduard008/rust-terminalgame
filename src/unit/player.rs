use num::ToPrimitive;

use crate::{point::Point2d, traits::{Position, Round, ToU16}, unit::Direction};

#[derive(Default, Clone)]
pub struct Player {
  position: Point2d<f32>,
  direction: Direction,
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
    if self.health > 0 {
      self.health -= damage;
    }
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
    let direction = self.direction.as_coordinates();
    self.position.x += direction.0 * self.speed.to_f32().unwrap();
    self.position.y += direction.1 * self.speed.to_f32().unwrap();
  }
  
  pub fn forward_position(&self) -> Point2d<u16> {
    let direction = self.direction.as_coordinates();
    let next_x = self.position.x + (direction.0 * self.speed.to_f32().unwrap());
    let next_y = self.position.y + (direction.1 * self.speed.to_f32().unwrap());
    Point2d::new(next_x, next_y).round().to_u16()
  }

  pub fn turn_left(&mut self) {
    self.direction -= 1;
  }

  pub fn turn_right(&mut self) {
    self.direction += 1;
  }
}

impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let direction_arrow = self.direction.to_string();
    write!(f, "{direction_arrow}")
  }
}

#[derive(Default)]
pub struct PlayerBuilder {
  position: Point2d<f32>,
  direction: Direction,
  speed: f64,
  health: u8
}

impl PlayerBuilder {
  pub fn new() -> Self {
    Self {
      position: Point2d::new(1.0, 2.0),
      direction: Direction::South,
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

  pub fn direction(mut self, x: f32, y: f32) -> Self {
    let point = Point2d::new(x, y);
    self.direction = Direction::point_as_direction(point);
    self
  }
}