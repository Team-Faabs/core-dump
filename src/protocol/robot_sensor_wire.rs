// ? Size of message:
//?   - Size of message: 1,817 Bytes
//?   - Throughput at 500Hz: 1,817 Bytes * 500 = 908.5kB/s
//?   - Throughput at 1000Hz: 1,817 Bytes * 1000 = 1.817MB/s
//?
//? This is all without wrapper message
//? Should be more than suitable for WiFi communication with a channel width of 40MHz

//? This is the message containing
//? all of the onboard sensor data
//?
//? It is sent over wifi, because latency
//? is not that important and more than
//? acceptable when using WiFi 7 6GHz

use serde::{Deserialize, Serialize};

/// Message containing all the onboard Sensor data:
///   - Lidar
///   - Vision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotSensorWire {
  pub robot_id: u8,
  #[serde(with = "postcard::fixint::le")]
  pub seq: u32,

  // Vision
  #[serde(with = "postcard::fixint::le")]
  pub ball_x: i32,
  #[serde(with = "postcard::fixint::le")]
  pub ball_y: i32,
  pub ball_size: f32,

  // Lidar
  /// 0 to 360 degrees, each degree one distance measurement in mm
  /// Resolution is 0.8 degrees => 450 Measurements
  #[serde(with = "crate::protocol::helpers::fixint_array::u32_le")]
  pub lidar_dist: [u32; 450],
}

impl RobotSensorWire {
  pub const ENCODED_LEN: usize = 1 + 4 + 4 + 4 + 4 + (4 * 450);

  #[inline]
  pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
    let mut message = [0; Self::ENCODED_LEN];
    let encoded = postcard::to_slice(self, &mut message)
      .expect("RobotSensorWire should fit in its fixed postcard buffer");
    debug_assert_eq!(encoded.len(), Self::ENCODED_LEN);
    message
  }

  #[inline]
  pub fn decode(message: [u8; Self::ENCODED_LEN]) -> Result<RobotSensorWire, postcard::Error> {
    match postcard::from_bytes(&message) {
      Ok(robot_sensor_wire) => Ok(robot_sensor_wire),
      Err(err) => Err(err),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roundtrips_fixed_size_sensor_wire() {
    let sensor = RobotSensorWire {
      robot_id: u8::MAX,
      seq: u32::MAX,
      ball_x: i32::MIN,
      ball_y: i32::MAX,
      ball_size: 42.5,
      lidar_dist: [u32::MAX; 450],
    };

    let encoded = sensor.encode();

    assert_eq!(encoded.len(), RobotSensorWire::ENCODED_LEN);
    assert_eq!(RobotSensorWire::decode(encoded), Ok(sensor));
  }

  #[test]
  fn sensor_decode_rejects_trailing_bytes() {
    let sensor = RobotSensorWire {
      robot_id: 1,
      seq: 2,
      ball_x: 3,
      ball_y: 4,
      ball_size: 5.0,
      lidar_dist: [6; 450],
    };
    let mut encoded = sensor.encode();
    encoded[0] = 0;

    assert!(RobotSensorWire::decode(encoded).is_err());
  }
}
