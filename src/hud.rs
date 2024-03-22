use crate::unit::Player;

pub struct Hud<'a> {
  score: u32,
  player: &'a Player,
  y_position: u16
}

impl<'a> Hud<'a> {
  pub fn new(score: u32, player: &'a Player, y_position: u16) -> Hud<'a> {
    Self {
      score, player, y_position
    }
  }

  pub fn draw(&self, buffer: &mut Vec<u8>) {
      crossterm::queue!(
          buffer,
          crossterm::cursor::MoveTo(
              0,
              self.y_position + 2),
          crossterm::style::Print(self)
      )
      .unwrap();
  }
}

impl<'a> std::fmt::Display for Hud<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Score: {:}, Health: {:.2}, Speed: {:.2}", self.score, self.player.health(), self.player.speed())
  }
}