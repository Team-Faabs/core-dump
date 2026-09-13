use num_traits::Float;

/// A proportional-integral-derivative controller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidController<T> {
  pub kp: T,
  pub ki: T,
  pub kd: T,
  integral: T,
  previous_error: Option<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidConfig<T> {
  pub kp: T,
  pub ki: T,
  pub kd: T,

}

impl<T: Float> PidController<T> {
  pub fn new(kp: T, ki: T, kd: T) -> Self {
    Self {
      kp,
      ki,
      kd,
      integral: T::zero(),
      previous_error: None,
    }
  }

  pub fn from_config(config: PidConfig<T>) -> Self {
    Self::new(config.kp, config.ki, config.kd)
  }

  /// Updates the controller from the current error and elapsed time.
  ///
  /// The derivative term is zero on the first update. A non-positive `dt`
  /// skips the integral and derivative terms.
  pub fn update(&mut self, error: T, dt: T) -> T {
    integrate(&mut self.integral, error, dt);
    let derivative = differentiate(&mut self.previous_error, error, dt);
    self.kp * error + self.ki * self.integral + self.kd * derivative
  }

  pub fn integral(&self) -> T {
    self.integral
  }

  pub fn previous_error(&self) -> Option<T> {
    self.previous_error
  }

  pub fn reset(&mut self) {
    self.integral = T::zero();
    self.previous_error = None;
  }
}

/// A proportional-integral controller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PiController<T> {
  pub kp: T,
  pub ki: T,
  integral: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PiConfig<T> {
  pub kp: T,
  pub ki: T,
}

impl<T: Float> PiController<T> {
  pub fn new(kp: T, ki: T) -> Self {
    Self {
      kp,
      ki,
      integral: T::zero(),
    }
  }

  pub fn from_config(config: PiConfig<T>) -> Self {
    Self::new(config.kp, config.ki)
  }

  /// Updates the controller from the current error and elapsed time.
  ///
  /// A non-positive `dt` skips integration.
  pub fn update(&mut self, error: T, dt: T) -> T {
    integrate(&mut self.integral, error, dt);
    self.kp * error + self.ki * self.integral
  }

  pub fn integral(&self) -> T {
    self.integral
  }

  pub fn reset(&mut self) {
    self.integral = T::zero();
  }
}

/// A proportional-derivative controller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PdController<T> {
  pub kp: T,
  pub kd: T,
  previous_error: Option<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PdConfig<T> {
  pub kp: T,
  pub kd: T,
}

impl<T: Float> PdController<T> {
  pub fn new(kp: T, kd: T) -> Self {
    Self {
      kp,
      kd,
      previous_error: None,
    }
  }

  pub fn from_config(config: PdConfig<T>) -> Self {
    Self::new(config.kp, config.kd)
  }

  /// Updates the controller from the current error and elapsed time.
  ///
  /// The derivative term is zero on the first update and whenever `dt` is not
  /// positive.
  pub fn update(&mut self, error: T, dt: T) -> T {
    let derivative = differentiate(&mut self.previous_error, error, dt);
    self.kp * error + self.kd * derivative
  }

  pub fn previous_error(&self) -> Option<T> {
    self.previous_error
  }

  pub fn reset(&mut self) {
    self.previous_error = None;
  }
}

fn integrate<T: Float>(integral: &mut T, error: T, dt: T) {
  if dt > T::zero() {
    *integral = *integral + error * dt;
  }
}

fn differentiate<T: Float>(previous_error: &mut Option<T>, error: T, dt: T) -> T {
  let derivative = if dt > T::zero() {
    previous_error.map_or(T::zero(), |previous| (error - previous) / dt)
  } else {
    T::zero()
  };
  *previous_error = Some(error);
  derivative
}
