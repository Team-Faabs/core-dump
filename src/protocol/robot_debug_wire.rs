use serde::{Deserialize, Serialize};
//? Size of message:
//?   - Size of message: 36 Bytes
//?   - Size for 12 robots: 36 Bytes * 12 = 432 Bytes
//?   - Throughput at 100Hz: 432 Bytes * 100 = 43.2kB/s
//?
//? This is all without wrapper message
//? More than suitable for WiFi communication with a channel width of 40MHz

//? This is the message containing
//? all of the onboard sensor data
//?
//? It is sent over wifi, because latency
//? is not that important and more than
//? acceptable when using WiFi 7 6GHz

/// Message from the robot containing debug information
/// todo!("Find more debug data to send back");
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RobotDebugWire {
  pub robot_id: u8,

  pub motor_current: [u8; 5],
  #[serde(with = "crate::protocol::helpers::fixint_array::u16_le")]
  pub motor_rotations: [u16; 5],
  #[serde(with = "crate::protocol::helpers::fixint_array::u16_le")]
  pub motor_encoder_rotations: [u16; 4], // Dribbler has no encoder

  /// Temps
  ///   - 1-5: ESC 1-5
  ///   - 6: Power Board
  ///   - 7: Kicker Board
  ///   - 8: Coil_1
  ///   - 9: Coil_2
  ///   - 10: CM5
  ///   - 11: Control Board
  ///   - 12: SDR
  pub temps: [u8; 12],

  /// Accelerometer data
  pub g_forces: u8,
}

impl RobotDebugWire {
  pub const ENCODED_LEN: usize = 1 + 5 + (2 * 5) + (2 * 4) + 12;

  #[inline]
  pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
    let mut message = [0; Self::ENCODED_LEN];
    let encoded = postcard::to_slice(self, &mut message)
      .expect("RobotDebugWire should fit in its fixed postcard buffer");
    debug_assert_eq!(encoded.len(), Self::ENCODED_LEN);
    message
  }

  #[inline]
  pub fn decode(message: [u8; Self::ENCODED_LEN]) -> Result<RobotDebugWire, postcard::Error> {
    match postcard::from_bytes(&message) {
      Ok(robot_debug_wire) => Ok(robot_debug_wire),
      Err(err) => Err(err),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roundtrips_fixed_size_debug_wire() {
    let debug = RobotDebugWire {
      robot_id: u8::MAX,
      motor_current: [u8::MAX; 5],
      motor_rotations: [u16::MAX; 5],
      motor_encoder_rotations: [u16::MAX; 4],
      temps: [u8::MAX; 12],
      g_forces: u8::MAX,
    };

    let encoded = debug.encode();

    assert_eq!(encoded.len(), RobotDebugWire::ENCODED_LEN);
    assert_eq!(RobotDebugWire::decode(encoded), Ok(debug));
  }

  #[test]
  fn debug_decode_rejects_trailing_bytes() {
    let debug = RobotDebugWire {
      robot_id: 1,
      motor_current: [2; 5],
      motor_rotations: [3; 5],
      motor_encoder_rotations: [4; 4],
      temps: [5; 12],
      g_forces: u8::MAX,
    };
    let mut encoded = debug.encode();
    encoded[0] = 0;

    assert!(RobotDebugWire::decode(encoded).is_err());
  }
}
