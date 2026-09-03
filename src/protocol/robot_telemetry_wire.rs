use serde::{Deserialize, Serialize};

//? Size of message:
//?   - Size of message: 17 Bytes
//?   - Size for 12 robots: 17 * 12 = 204 Bytes
//?   - Throughput at 500Hz: 204 Bytes * 500 = 102kB/s
//?   - Throughput at 1000Hz: 204 Bytes * 1000 = 204kB/s
//?
//? This is all without wrapper message

/// This is a small debug message from the robot to the CrashPilot
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RobotTelemetryWire {
  pub robot_id: u8,
  pub status: u8,

  #[serde(with = "postcard::fixint::le")]
  pub seq_seen: u32,

  // Data from the onboard imu
  #[serde(with = "postcard::fixint::le")]
  pub vx_mmps: i16,
  #[serde(with = "postcard::fixint::le")]
  pub vy_mmps: i16,
  #[serde(with = "postcard::fixint::le")]
  pub orientation: u16,

  pub battery_mv: u8,
  pub current: u8,
  pub capacitor_v: u8,

  /// Flags from the robot to the CrashPilot
  ///   - Bit 1: Has Ball
  ///   - Bit 2: Reserved
  ///   - Bit 3: Reserved
  ///   - Bit 4: Reserved
  ///   - Bit 5: Fault ESC 1
  ///   - Bit 6: Fault ESC 2
  ///   - Bit 7: Fault ESC 3
  ///   - Bit 8: Fault ESC 4
  ///   - Bit 9: Fault ESC 5
  ///   - Bit 10: Fault Kicker
  ///   - Bit 11: Fault Vision
  ///   - Bit 12: Fault Wi-Fi
  ///   - Bit 13: Fault Battery
  ///   - Bit 14:
  ///   - Bit 15:
  ///   - Bit 16: Shutting down
  #[serde(with = "postcard::fixint::le")]
  pub flags: u16,
}

impl RobotTelemetryWire {
  pub const ENCODED_LEN: usize = 17;

  #[inline]
  pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
    let mut message = [0; Self::ENCODED_LEN];
    let encoded = postcard::to_slice(self, &mut message)
      .expect("RobotTelemetryWire should fit in its fixed postcard buffer");
    debug_assert_eq!(encoded.len(), Self::ENCODED_LEN);
    message
  }

  #[inline]
  pub fn decode(message: [u8; Self::ENCODED_LEN]) -> Result<RobotTelemetryWire, postcard::Error> {
    match postcard::from_bytes(&message) {
      Ok(robot_telemetry_wire) => Ok(robot_telemetry_wire),
      Err(err) => Err(err),
    }
  }

  #[inline]
  pub fn has_ball(&self) -> bool {
    self.flags & (1 << 0) != 0
  }
  // #[inline]
  // pub fn reserved_bit_2(&self) -> bool {
  //   self.flags & (1 << 1) != 0
  // }
  // #[inline]
  // pub fn reserved_bit_3(&self) -> bool {
  //   self.flags & (1 << 2) != 0
  // }
  // #[inline]
  // pub fn reserved_bit_4(&self) -> bool {
  //   self.flags & (1 << 3) != 0
  // }
  #[inline]
  pub fn esc_fault(&self) -> Option<u8> {
    for i in 0..5 {
      if self.flags & (1 << (4 + i)) != 0 {
        return Some(i + 1);
      }
    }
    None
  }
  #[inline]
  pub fn fault_kicker(&self) -> bool {
    self.flags & (1 << 9) != 0
  }
  #[inline]
  pub fn fault_vision(&self) -> bool {
    self.flags & (1 << 10) != 0
  }
  #[inline]
  pub fn fault_wifi(&self) -> bool {
    self.flags & (1 << 11) != 0
  }
  #[inline]
  pub fn fault_battery(&self) -> bool {
    self.flags & (1 << 12) != 0
  }
  // #[inline]
  // pub fn bit14(&self) -> bool {
  //   self.flags & (1 << 13) != 0
  // }
  // #[inline]
  // pub fn bit15(&self) -> bool {
  //   self.flags & (1 << 14) != 0
  // }
  #[inline]
  pub fn is_shutting_down(&self) -> bool {
    self.flags & (1 << 15) != 0
  }

  #[inline]
  pub fn new() -> Self {
    Self {
      robot_id: 0,
      status: 0,
      seq_seen: 0,
      vx_mmps: 0,
      vy_mmps: 0,
      orientation: 0,
      battery_mv: 0,
      current: 0,
      capacitor_v: 0,
      flags: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roundtrips_fixed_size_telemetry() {
    let telemetry = RobotTelemetryWire {
      robot_id: u8::MAX,
      status: u8::MAX,
      seq_seen: u32::MAX,
      vx_mmps: i16::MIN,
      vy_mmps: i16::MAX,
      orientation: u16::MAX,
      battery_mv: u8::MAX,
      current: u8::MAX,
      capacitor_v: u8::MAX,
      flags: u16::MAX,
    };

    let encoded = telemetry.encode();

    assert_eq!(encoded.len(), RobotTelemetryWire::ENCODED_LEN);
    assert_eq!(RobotTelemetryWire::decode(encoded), Ok(telemetry));
  }
}
