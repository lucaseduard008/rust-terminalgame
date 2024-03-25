use num::traits::NumAssign;
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};
use std::ops::{Add, AddAssign, Mul, Range, Sub};

use crate::point::Point2d;

pub trait Position<T: NumAssign + Copy> {
    fn position(&self) -> Point2d<T>;
    fn set_position(&mut self, position: Point2d<T>);
    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let new_position = Point2d::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}

pub trait ToU16 {
    fn to_u16(&self) -> Point2d<u16>;
}

pub trait Round {
    fn round(&self) -> Self;
}

impl ToU16 for Point2d<f32> {
    fn to_u16(&self) -> Point2d<u16> {
        Point2d {
            x: self.x as u16,
            y: self.y as u16
        }
    }
}

impl Round for Point2d<f32> {
    fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round()
        }
    }
}

impl<T: NumAssign + Copy> Position<T> for Point2d<T> {
    fn position(&self) -> Point2d<T> {
        *self
    }

    fn set_position(&mut self, position: Point2d<T>) {
        *self = position;
    }
}

impl<T: NumAssign + Copy> PartialEq<T> for Point2d<T> {
    fn eq(&self, other: &T) -> bool {
        if self == other {
            return true;
        }
        false
    }
}

impl<T: Add<Output = T>> Add for Point2d<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point2d<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point2d<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point2d<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}