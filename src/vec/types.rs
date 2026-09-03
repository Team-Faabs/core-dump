use crate::proto::{Vector2, Vector3};
use num_traits::{Float, One, Zero};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

/// A fixed-size vector backed by an array.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Vector<T, N> {
  pub const fn new(values: [T; N]) -> Self {
    Self(values)
  }

  pub fn into_inner(self) -> [T; N] {
    self.0
  }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
  fn from(values: [T; N]) -> Self {
    Self(values)
  }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

/// A row-major fixed-size matrix backed by nested arrays.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize>(pub [[T; COLS]; ROWS]);

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
  pub const fn new(values: [[T; COLS]; ROWS]) -> Self {
    Self(values)
  }

  pub fn into_inner(self) -> [[T; COLS]; ROWS] {
    self.0
  }
}

impl<T: Copy + Zero, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
  pub fn transpose(self) -> Matrix<T, COLS, ROWS> {
    let mut result = Matrix::new([[T::zero(); ROWS]; COLS]);
    for row in 0..ROWS {
      for column in 0..COLS {
        result[column][row] = self[row][column];
      }
    }
    result
  }
}

impl<T: Copy + Zero + One, const N: usize> Matrix<T, N, N> {
  pub fn identity() -> Self {
    let mut result = Self::new([[T::zero(); N]; N]);
    for index in 0..N {
      result[index][index] = T::one();
    }
    result
  }
}

impl<T: Float, const N: usize> Matrix<T, N, N> {
  pub(crate) fn inverse(mut self) -> Option<Self> {
    let mut result = Self::identity();

    for column in 0..N {
      let mut pivot = column;
      for row in column + 1..N {
        if self[row][column].abs() > self[pivot][column].abs() {
          pivot = row;
        }
      }
      if self[pivot][column] == T::zero() || !self[pivot][column].is_finite() {
        return None;
      }

      self.0.swap(column, pivot);
      result.0.swap(column, pivot);

      let divisor = self[column][column];
      for index in 0..N {
        self[column][index] = self[column][index] / divisor;
        result[column][index] = result[column][index] / divisor;
      }

      for row in 0..N {
        if row == column {
          continue;
        }
        let factor = self[row][column];
        for index in 0..N {
          self[row][index] = self[row][index] - factor * self[column][index];
          result[row][index] = result[row][index] - factor * result[column][index];
        }
      }
    }

    Some(result)
  }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
  fn from(values: [[T; COLS]; ROWS]) -> Self {
    Self(values)
  }
}

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
  type Output = [T; COLS];

  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

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
