use std::ops::{Add, Sub, AddAssign, SubAssign};

use crate::point::Point2d;

#[derive(Debug, Clone, Copy, Default)]
pub enum Direction {
  #[default]
  North = 0,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest
}


impl Direction {
  pub fn as_coordinates(&self) -> (f32, f32) {
    match self {
      Direction::North => (0.0, -1.0),
      Direction::NorthEast => (0.7061, -0.7061),
      Direction::East => (1.0, 0.0),
      Direction::SouthEast => (0.7061, 0.7061),
      Direction::South => (0.0, 1.0),
      Direction::SouthWest => (-0.7061, 0.7061),
      Direction::West => (-1.0, 0.0),
      Direction::NorthWest => (-0.7061, -0.7061)
    }
  }
  // directions: ↑↗→↘↓↙←↖
  pub fn point_as_direction(point: Point2d<f32>) -> Self {
    match point {
      Point2d { x, y } if x == 0.0 && y == -1.0 => Direction::North,
      Point2d { x, y } if x == 0.7061 && y == -0.7061 => Direction::NorthEast,
      Point2d { x, y } if x == 1.0 && y == 0.0 => Direction::East,
      Point2d { x, y } if x == 0.7061 && y == 0.7061 => Direction::SouthEast,
      Point2d { x, y } if x == 0.0 && y == 1.0 => Direction::South,
      Point2d { x, y } if x == -0.7061 && y == 0.7061 => Direction::SouthWest,
      Point2d { x, y } if x == -1.0 && y == 0.0 => Direction::West,
      Point2d { x, y } if x == -0.7061 && y == -0.7061 => Direction::NorthWest,
      _ => unreachable!()
    }
  }
}

impl std::fmt::Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Direction::North => write!(f, "↑"),
        Direction::NorthEast => write!(f, "↗"),
        Direction::East => write!(f, "→"),
        Direction::SouthEast => write!(f, "↘"),
        Direction::South => write!(f, "↓"),
        Direction::SouthWest => write!(f, "↙"),
        Direction::West => write!(f, "←"),
        Direction::NorthWest => write!(f, "↖"),
      }
  }
}

impl Add<i32> for Direction {
 type Output = Self;

 fn add(self, rhs: i32) -> Self {
  let result_value = (self as i32 + rhs) % 8;
  match result_value {
      0 => Direction::North,
      1 => Direction::NorthEast,
      2 => Direction::East,
      3 => Direction::SouthEast,
      4 => Direction::South,
      5 => Direction::SouthWest,
      6 => Direction::West,
      7 => Direction::NorthWest,
      _ => unreachable!(),
  }
}
}

impl Sub<i32> for Direction {
  type Output = Self;

  fn sub(self, rhs: i32) -> Self::Output {
    let diff = (self as i32 - rhs + 8) % 8;
        match diff {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::East,
            3 => Direction::SouthEast,
            4 => Direction::South,
            5 => Direction::SouthWest,
            6 => Direction::West,
            7 => Direction::NorthWest,
            _ => unreachable!(),
        }
  }
}

impl AddAssign<i32> for Direction {
  fn add_assign(&mut self, rhs: i32) {
      *self = *self + rhs;
  }
}

impl SubAssign<i32> for Direction {
  fn sub_assign(&mut self, rhs: i32) {
      *self = *self - rhs;
  }
}