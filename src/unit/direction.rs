#[derive(Default)]
pub struct DirectionStruct<T> {
  pub x: T,
  pub y: T
}

impl<T> DirectionStruct<T> {
  fn as_coordinates(&self) -> Self {
    todo!()
  }
}
