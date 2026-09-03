//! Allocation-free Kalman filters for fixed-size systems.

use num_traits::Float;

pub use crate::vec::types::{Matrix, Vector};

/// An error produced while applying a filter update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KalmanError {
  /// The innovation covariance could not be inverted.
  SingularInnovation,
}

/// A linear Kalman filter with a fixed-size state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KalmanFilter<T, const STATES: usize> {
  pub state: Vector<T, STATES>,
  pub covariance: Matrix<T, STATES, STATES>,
}

impl<T: Float, const STATES: usize> KalmanFilter<T, STATES> {
  pub const fn new(state: Vector<T, STATES>, covariance: Matrix<T, STATES, STATES>) -> Self {
    Self { state, covariance }
  }

  /// Advances the state using `x = F x` and `P = F P Fᵀ + Q`.
  pub fn predict(
    &mut self,
    transition: &Matrix<T, STATES, STATES>,
    process_noise: &Matrix<T, STATES, STATES>,
  ) {
    self.state = *transition * self.state;
    self.covariance = predict_covariance(&self.covariance, transition, process_noise);
  }

  /// Advances the state with a control input using `x = F x + B u`.
  pub fn predict_with_control<const INPUTS: usize>(
    &mut self,
    transition: &Matrix<T, STATES, STATES>,
    process_noise: &Matrix<T, STATES, STATES>,
    control_model: &Matrix<T, STATES, INPUTS>,
    control: &Vector<T, INPUTS>,
  ) {
    self.state = *transition * self.state + *control_model * *control;
    self.covariance = predict_covariance(&self.covariance, transition, process_noise);
  }

  /// Incorporates a measurement using the observation model `H` and noise `R`.
  pub fn update<const MEASUREMENTS: usize>(
    &mut self,
    measurement: &Vector<T, MEASUREMENTS>,
    observation: &Matrix<T, MEASUREMENTS, STATES>,
    measurement_noise: &Matrix<T, MEASUREMENTS, MEASUREMENTS>,
  ) -> Result<(), KalmanError> {
    let innovation = *measurement - *observation * self.state;
    update(
      &mut self.state,
      &mut self.covariance,
      &innovation,
      observation,
      measurement_noise,
    )
  }
}

/// An extended Kalman filter with a fixed-size state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtendedKalmanFilter<T, const STATES: usize> {
  pub state: Vector<T, STATES>,
  pub covariance: Matrix<T, STATES, STATES>,
}

impl<T: Float, const STATES: usize> ExtendedKalmanFilter<T, STATES> {
  pub const fn new(state: Vector<T, STATES>, covariance: Matrix<T, STATES, STATES>) -> Self {
    Self { state, covariance }
  }

  /// Advances the state through a nonlinear transition function.
  ///
  /// The function receives the prior state and returns the predicted state and
  /// the transition Jacobian evaluated at that prior state.
  pub fn predict<F>(&mut self, transition: F, process_noise: &Matrix<T, STATES, STATES>)
  where
    F: FnOnce(&Vector<T, STATES>) -> (Vector<T, STATES>, Matrix<T, STATES, STATES>),
  {
    let (state, jacobian) = transition(&self.state);
    self.covariance = predict_covariance(&self.covariance, &jacobian, process_noise);
    self.state = state;
  }

  /// Incorporates a measurement through a nonlinear observation function.
  ///
  /// The function returns the predicted measurement and the observation
  /// Jacobian, both evaluated at the current state.
  pub fn update<const MEASUREMENTS: usize, H>(
    &mut self,
    measurement: &Vector<T, MEASUREMENTS>,
    measurement_noise: &Matrix<T, MEASUREMENTS, MEASUREMENTS>,
    observation: H,
  ) -> Result<(), KalmanError>
  where
    H: FnOnce(&Vector<T, STATES>) -> (Vector<T, MEASUREMENTS>, Matrix<T, MEASUREMENTS, STATES>),
  {
    let (predicted, jacobian) = observation(&self.state);
    let innovation = *measurement - predicted;
    update(
      &mut self.state,
      &mut self.covariance,
      &innovation,
      &jacobian,
      measurement_noise,
    )
  }
}

fn update<T: Float, const STATES: usize, const MEASUREMENTS: usize>(
  state: &mut Vector<T, STATES>,
  covariance: &mut Matrix<T, STATES, STATES>,
  innovation: &Vector<T, MEASUREMENTS>,
  observation: &Matrix<T, MEASUREMENTS, STATES>,
  measurement_noise: &Matrix<T, MEASUREMENTS, MEASUREMENTS>,
) -> Result<(), KalmanError> {
  let cross_covariance = *covariance * observation.transpose();
  let innovation_covariance = *observation * cross_covariance + *measurement_noise;
  let innovation_covariance_inv = innovation_covariance
    .inverse()
    .ok_or(KalmanError::SingularInnovation)?;
  let gain = cross_covariance * innovation_covariance_inv;

  *state = *state + gain * *innovation;

  // Joseph form is slightly more work than (I - KH)P, but remains symmetric
  // and positive semi-definite under ordinary floating-point roundoff.
  let residual = Matrix::identity() - gain * *observation;
  *covariance =
    residual * *covariance * residual.transpose() + gain * *measurement_noise * gain.transpose();
  Ok(())
}

fn predict_covariance<T: Float, const N: usize>(
  covariance: &Matrix<T, N, N>,
  transition: &Matrix<T, N, N>,
  process_noise: &Matrix<T, N, N>,
) -> Matrix<T, N, N> {
  *transition * *covariance * transition.transpose() + *process_noise
}