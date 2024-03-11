#[derive(Default)]
pub struct Player {
  speed: f64,
  health: u8
}

impl Player {
  pub fn new() -> PlayerBuilder {
    PlayerBuilder::default()
  }

  pub fn is_alive(&self) -> bool {
    if self.health > 0 {
      return true;
    }
    return false;
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
  speed: f64,
  health: u8
}

impl PlayerBuilder {
  pub fn new() -> Self {
    Self {
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
      speed: self.speed,
      health: self.health
    }
  }
}