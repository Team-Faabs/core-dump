use serde::{Deserialize, Serialize};

//? Size of message:
//?   - Size of message: 10 Bytes
//?   - Size for 12 robots: 12 * 10 = 120 Bytes
//?   - Throughput at 500Hz: 120 Bytes * 500 = 60kB/s
//?   - Throughput at 1000Hz: 120 Bytes * 1000 = 120kB/s
//?
//? This is all without wrapper message

/// Robot Control Protocol
/// Sends basic data to each robot
/// to control movement, etc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RobotCommandWire {
  pub intent: u8,

  // Velocity in mm/s
  #[serde(with = "postcard::fixint::le")]
  pub vx_mmps: i16,
  #[serde(with = "postcard::fixint::le")]
  pub vy_mmps: i16,
  // Rotation in mrad/s
  #[serde(with = "postcard::fixint::le")]
  pub omega_mradps: i16,

  pub kick_speed: u8,
  pub dribbler_speed: u8,

  /// Flags to execute specific robot stuff
  ///   - Bit 1: Kick
  ///   - Bit 2: Chip
  ///   - Bit 3: Dribbler
  ///   - Bit 4:
  ///   - Bit 5:
  ///   - Bit 6:
  ///   - Bit 7:
  ///   - Bit 8: Shutdown
  pub flags: u8,
}

// just a theoretical version, probably not practical in production
#[allow(dead_code)]
struct PackedCommand {
  intent: u8,

  // x and y velocity in 11bits + 1 sign bit resolution. Value * 4mm/s
  vx_mps: u8,
  vxy_mps: u8,
  vy_mps: u8,

  // stored in -128..127 converted to mrad/s via 360*(omega_mradps/128)^3
  omega_mradps: i8,

  // 6bit for kick speed, 2bit for dribbler speed
  kick_speed: u8,

  // 4 flags + 2 bit for the dribbler speed
  flags: u8,
}

impl RobotCommandWire {
  pub const ENCODED_LEN: usize = 10;

  #[inline]
  pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
    let mut message = [0; Self::ENCODED_LEN];
    let encoded = postcard::to_slice(self, &mut message)
      .expect("RobotCommandWire should fit in its fixed postcard buffer");
    debug_assert_eq!(encoded.len(), Self::ENCODED_LEN);
    message
  }

  #[inline]
  pub fn decode(message: [u8; Self::ENCODED_LEN]) -> Self {
    postcard::from_bytes(&message).expect("RobotCommandWire fixed buffer should decode")
  }

  #[inline]
  pub fn kick(&self) -> bool {
    self.flags & (1 << 0) != 0
  }
  #[inline]
  pub fn chip(&self) -> bool {
    self.flags & (1 << 1) != 0
  }
  #[inline]
  pub fn dribbler(&self) -> bool {
    self.flags & (1 << 2) != 0
  }
  // #[inline]
  // pub fn bit4(&self) -> bool {
  //   self.flags & (1 << 4) != 0
  // }
  // #[inline]
  // pub fn bit5(&self) -> bool {
  //   self.flags & (1 << 5) != 0
  // }
  // #[inline]
  // pub fn bit6(&self) -> bool {
  //   self.flags & (1 << 6) != 0
  // }
  // #[inline]
  // pub fn bit7(&self) -> bool {
  //   self.flags & (1 << 7) != 0
  // }
  #[inline]
  pub fn shutdown(&self) -> bool {
    self.flags & (1 << 7) != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roundtrips_fixed_size_command() {
    let command = RobotCommandWire {
      intent: u8::MAX,
      vx_mmps: i16::MIN,
      vy_mmps: i16::MAX,
      omega_mradps: -1234,
      kick_speed: u8::MAX,
      dribbler_speed: u8::MAX,
      flags: u8::MAX,
    };

    let encoded = command.encode();

    assert_eq!(encoded.len(), RobotCommandWire::ENCODED_LEN);
    assert_eq!(RobotCommandWire::decode(encoded), command);
  }
}
