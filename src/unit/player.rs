#[derive(Default)]
pub struct Player {
  speed: f32,
  health: u32
}

impl Player {
  pub fn builder() -> PlayerBuilder {
    PlayerBuilder::default()
  }
}

#[derive(Default)]
pub struct PlayerBuilder {
}

impl PlayerBuilder {

}