use crate::vec::types::{Matrix, Vec2, Vec3, Vector};
use num_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};

impl<T> Add for Vec2<T>
where
  T: Add<Output = T>,
{
  type Output = Vec2<T>;

  fn add(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl<T> Sub for Vec2<T>
where
  T: Sub<Output = T>,
{
  type Output = Vec2<T>;

  fn sub(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl<T> Mul for Vec2<T>
where
  T: Mul<Output = T>,
{
  type Output = Vec2<T>;

  fn mul(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x * rhs.x, self.y * rhs.y)
  }
}

impl<T> Mul<T> for Vec2<T>
where
  T: Copy + Mul<Output = T>,
{
  type Output = Vec2<T>;

  fn mul(self, rhs: T) -> Self::Output {
    Vec2::new(self.x * rhs, self.y * rhs)
  }
}

impl<T> Div for Vec2<T>
where
  T: Div<Output = T>,
{
  type Output = Vec2<T>;

  fn div(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x / rhs.x, self.y / rhs.y)
  }
}

impl<T> Div<T> for Vec2<T>
where
  T: Copy + Div<Output = T>,
{
  type Output = Vec2<T>;

  fn div(self, rhs: T) -> Self::Output {
    Vec2::new(self.x / rhs, self.y / rhs)
  }
}

impl<T> Add for Vec3<T>
where
  T: Add<Output = T>,
{
  type Output = Vec3<T>;

  fn add(self, rhs: Self) -> Self::Output {
    Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T> Sub for Vec3<T>
where
  T: Sub<Output = T>,
{
  type Output = Vec3<T>;

  fn sub(self, rhs: Self) -> Self::Output {
    Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl<T> Mul for Vec3<T>
where
  T: Mul<Output = T>,
{
  type Output = Vec3<T>;

  fn mul(self, rhs: Self) -> Self::Output {
    Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl<T> Mul<T> for Vec3<T>
where
  T: Copy + Mul<Output = T>,
{
  type Output = Vec3<T>;

  fn mul(self, rhs: T) -> Self::Output {
    Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl<T> Div<T> for Vec3<T>
where
  T: Copy + Div<Output = T>,
{
  type Output = Vec3<T>;

  fn div(self, rhs: T) -> Self::Output {
    Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl<T, const N: usize> Add for Vector<T, N>
where
  T: Copy + Add<Output = T>,
{
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self::Output {
    for index in 0..N {
      self[index] = self[index] + rhs[index];
    }
    self
  }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
  T: Copy + Sub<Output = T>,
{
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self::Output {
    for index in 0..N {
      self[index] = self[index] - rhs[index];
    }
    self
  }
}

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
  T: Copy + Add<Output = T>,
{
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self::Output {
    for row in 0..ROWS {
      for column in 0..COLS {
        self[row][column] = self[row][column] + rhs[row][column];
      }
    }
    self
  }
}

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
  T: Copy + Sub<Output = T>,
{
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self::Output {
    for row in 0..ROWS {
      for column in 0..COLS {
        self[row][column] = self[row][column] - rhs[row][column];
      }
    }
    self
  }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<Vector<T, COLS>> for Matrix<T, ROWS, COLS>
where
  T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
  type Output = Vector<T, ROWS>;

  fn mul(self, rhs: Vector<T, COLS>) -> Self::Output {
    let mut result = Vector::new([T::zero(); ROWS]);
    for row in 0..ROWS {
      for column in 0..COLS {
        result[row] = result[row] + self[row][column] * rhs[column];
      }
    }
    result
  }
}

impl<T, const ROWS: usize, const INNER: usize, const COLS: usize> Mul<Matrix<T, INNER, COLS>>
  for Matrix<T, ROWS, INNER>
where
  T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
  type Output = Matrix<T, ROWS, COLS>;

  fn mul(self, rhs: Matrix<T, INNER, COLS>) -> Self::Output {
    let mut result = Matrix::new([[T::zero(); COLS]; ROWS]);
    for row in 0..ROWS {
      for column in 0..COLS {
        for index in 0..INNER {
          result[row][column] = result[row][column] + self[row][index] * rhs[index][column];
        }
      }
    }
    result
  }
}
