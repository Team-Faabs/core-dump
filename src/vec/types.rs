use crate::proto::{Vector2, Vector3};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect<T> {
  pub min: Vec2<T>,
  pub max: Vec2<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
  X,
  Y,
}

impl<T> Vec2<T> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl Vec2<f32> {
  #[inline]
  pub const fn new_from_ssl_vec2(v: Vector2) -> Vec2<f32> {
    Vec2::new(
      // Multiplication with `1000` to convert to mm from m
      v.x * 1000f32,
      v.y * 1000f32,
    )
  }

  #[inline]
  pub const fn new_from_ssl_vec3(v: Vector3) -> Vec2<f32> {
    Vec2::new(
      // Multiplication with `1000` to convert to mm from m
      v.x * 1000.0,
      v.y * 1000.0,
    )
  }

  #[inline]
  pub fn to_crashpilot_vec2(self) -> crate::proto::crashpilot::Vector2 {
    crate::proto::crashpilot::Vector2 {
      x: self.x as i32,
      y: self.y as i32,
    }
  }
}

impl<T: Default> Default for Vec2<T> {
  fn default() -> Self {
    Self {
      x: T::default(),
      y: T::default(),
    }
  }
}

impl<T: Zero> Vec2<T> {
  pub fn zero() -> Self {
    Self {
      x: T::zero(),
      y: T::zero(),
    }
  }
}

impl<T> Vec3<T> {
  #[inline]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }

  /// Drops the `z` component, projecting onto the XY plane.
  #[inline]
  pub fn xy(self) -> Vec2<T> {
    Vec2::new(self.x, self.y)
  }
}

impl<T: Default> Default for Vec3<T> {
  fn default() -> Self {
    Self {
      x: T::default(),
      y: T::default(),
      z: T::default(),
    }
  }
}

impl<T: Zero> Vec3<T> {
  pub fn zero() -> Self {
    Self {
      x: T::zero(),
      y: T::zero(),
      z: T::zero(),
    }
  }
}

pub type Mm = i16; // A millimeter in 0..32767 mm. 1 m = 1000 mm.
