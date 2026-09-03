use crate::proto::{HullColor, MatchType, Referee, referee};
use crate::protocol::robot_debug_wire::RobotDebugWire;
use crate::protocol::robot_sensor_wire::RobotSensorWire;
use crate::protocol::robot_telemetry_wire::RobotTelemetryWire;
use crate::types::cp_types::{Ball, Robot};
use serde::Serialize;

//? This message sends all the data wrappers to the
//? Interface
//?
//? The Interface is a Rust Crate now -> no Network Overhead
//? like before

/// Interface Data Wire
/// Contains all the necessary data that the
/// interface needs to display
#[derive(Debug, Clone)]
pub struct InterfaceDataWire {
  pub robots: [Option<Robot>; 32], // Maximum amount of robots on the field at the same time
  pub ball: Option<Ball>,          // Currently tracked Ball

  pub game_controller_data: Option<Referee>,

  //? Different debug data
  pub robot_telemetry_wire: [Option<RobotTelemetryWire>; 12],
  pub robot_debug_wire: [Option<RobotDebugWire>; 12],
  pub robot_sensor_wire: [Option<RobotSensorWire>; 12],
}

impl InterfaceDataWire {
  #[inline]
  pub fn encode(&self) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec(&InterfaceDataWireJson::from(self))
  }

  #[inline]
  pub fn encode_string(&self) -> serde_json::Result<String> {
    serde_json::to_string(&InterfaceDataWireJson::from(self))
  }
}

// Additional data the interface gets for debug purposes
pub struct InterfaceDebugData {
  // AI stuff and Phase/Pre States will go here
}

#[derive(Serialize)]
struct InterfaceDataWireJson<'a> {
  robots: &'a [Option<Robot>; 32],
  ball: &'a Option<Ball>,
  game_controller_data: Option<RefereeJson<'a>>,
  robot_telemetry_wire: Vec<Option<RobotTelemetryWireJson>>,
  robot_debug_wire: Vec<Option<RobotDebugWireJson<'a>>>,
  robot_sensor_wire: Vec<Option<RobotSensorWireJson<'a>>>,
}

impl<'a> From<&'a InterfaceDataWire> for InterfaceDataWireJson<'a> {
  fn from(message: &'a InterfaceDataWire) -> Self {
    Self {
      robots: &message.robots,
      ball: &message.ball,
      game_controller_data: message.game_controller_data.as_ref().map(RefereeJson::from),
      robot_telemetry_wire: message
        .robot_telemetry_wire
        .iter()
        .map(|telemetry| telemetry.as_ref().map(RobotTelemetryWireJson::from))
        .collect(),
      robot_debug_wire: message
        .robot_debug_wire
        .iter()
        .map(|debug| debug.as_ref().map(RobotDebugWireJson::from))
        .collect(),
      robot_sensor_wire: message
        .robot_sensor_wire
        .iter()
        .map(|sensor| sensor.as_ref().map(RobotSensorWireJson::from))
        .collect(),
    }
  }
}

#[derive(Serialize)]
struct RobotTelemetryWireJson {
  robot_id: u8,
  status: u8,
  seq_seen: u32,
  vx_mmps: i16,
  vy_mmps: i16,
  orientation: u16,
  battery_mv: u8,
  current: u8,
  capacitor_v: u8,
  flags: u16,
}

impl From<&RobotTelemetryWire> for RobotTelemetryWireJson {
  fn from(message: &RobotTelemetryWire) -> Self {
    Self {
      robot_id: message.robot_id,
      status: message.status,
      seq_seen: message.seq_seen,
      vx_mmps: message.vx_mmps,
      vy_mmps: message.vy_mmps,
      orientation: message.orientation,
      battery_mv: message.battery_mv,
      current: message.current,
      capacitor_v: message.capacitor_v,
      flags: message.flags,
    }
  }
}

#[derive(Serialize)]
struct RobotDebugWireJson<'a> {
  robot_id: u8,
  motor_current: &'a [u8; 5],
  motor_rotations: &'a [u16; 5],
  motor_encoder_rotations: &'a [u16; 4],
  temps: &'a [u8; 12],
}

impl<'a> From<&'a RobotDebugWire> for RobotDebugWireJson<'a> {
  fn from(message: &'a RobotDebugWire) -> Self {
    Self {
      robot_id: message.robot_id,
      motor_current: &message.motor_current,
      motor_rotations: &message.motor_rotations,
      motor_encoder_rotations: &message.motor_encoder_rotations,
      temps: &message.temps,
    }
  }
}

#[derive(Serialize)]
struct RobotSensorWireJson<'a> {
  robot_id: u8,
  seq: u32,
  ball_angle: i32,
  ball_dist: u32,
  lidar_dist: &'a [u32],
}

impl<'a> From<&'a RobotSensorWire> for RobotSensorWireJson<'a> {
  fn from(message: &'a RobotSensorWire) -> Self {
    Self {
      robot_id: message.robot_id,
      seq: message.seq,
      ball_angle: message.ball_angle,
      ball_dist: message.ball_dist,
      lidar_dist: &message.lidar_dist,
    }
  }
}

#[derive(Clone, Copy, Serialize)]
struct EnumJson {
  value: i32,
  name: Option<&'static str>,
}

#[derive(Serialize)]
struct PointJson {
  x: f32,
  y: f32,
}

impl From<&referee::Point> for PointJson {
  fn from(point: &referee::Point) -> Self {
    Self {
      x: point.x,
      y: point.y,
    }
  }
}

#[derive(Serialize)]
struct TeamInfoJson<'a> {
  name: &'a str,
  score: u32,
  red_cards: u32,
  yellow_card_times: &'a [u32],
  yellow_cards: u32,
  timeouts: u32,
  timeout_time: u32,
  goalkeeper: u32,
  foul_counter: Option<u32>,
  ball_placement_failures: Option<u32>,
  can_place_ball: Option<bool>,
  max_allowed_bots: Option<u32>,
  bot_substitution_intent: Option<bool>,
  ball_placement_failures_reached: Option<bool>,
  bot_substitution_allowed: Option<bool>,
  bot_substitutions_left: Option<u32>,
  bot_substitution_time_left: Option<u32>,
  hull_color: Option<EnumJson>,
}

impl<'a> From<&'a referee::TeamInfo> for TeamInfoJson<'a> {
  fn from(team: &'a referee::TeamInfo) -> Self {
    Self {
      name: &team.name,
      score: team.score,
      red_cards: team.red_cards,
      yellow_card_times: &team.yellow_card_times,
      yellow_cards: team.yellow_cards,
      timeouts: team.timeouts,
      timeout_time: team.timeout_time,
      goalkeeper: team.goalkeeper,
      foul_counter: team.foul_counter,
      ball_placement_failures: team.ball_placement_failures,
      can_place_ball: team.can_place_ball,
      max_allowed_bots: team.max_allowed_bots,
      bot_substitution_intent: team.bot_substitution_intent,
      ball_placement_failures_reached: team.ball_placement_failures_reached,
      bot_substitution_allowed: team.bot_substitution_allowed,
      bot_substitutions_left: team.bot_substitutions_left,
      bot_substitution_time_left: team.bot_substitution_time_left,
      hull_color: team.hull_color.map(hull_color_json),
    }
  }
}

#[derive(Serialize)]
struct RefereeJson<'a> {
  source_identifier: Option<&'a str>,
  match_type: Option<EnumJson>,
  packet_timestamp: u64,
  stage: EnumJson,
  stage_time_left: Option<i64>,
  command: EnumJson,
  command_counter: u32,
  command_timestamp: u64,
  yellow: TeamInfoJson<'a>,
  blue: TeamInfoJson<'a>,
  designated_position: Option<PointJson>,
  blue_team_on_positive_half: Option<bool>,
  next_command: Option<EnumJson>,
  game_event_count: usize,
  game_event_proposal_count: usize,
  current_action_time_remaining: Option<i64>,
  status_message: Option<&'a str>,
}

impl<'a> From<&'a Referee> for RefereeJson<'a> {
  fn from(referee: &'a Referee) -> Self {
    Self {
      source_identifier: referee.source_identifier.as_deref(),
      match_type: referee.match_type.map(match_type_json),
      packet_timestamp: referee.packet_timestamp,
      stage: stage_json(referee.stage),
      stage_time_left: referee.stage_time_left,
      command: command_json(referee.command),
      command_counter: referee.command_counter,
      command_timestamp: referee.command_timestamp,
      yellow: TeamInfoJson::from(&referee.yellow),
      blue: TeamInfoJson::from(&referee.blue),
      designated_position: referee.designated_position.as_ref().map(PointJson::from),
      blue_team_on_positive_half: referee.blue_team_on_positive_half,
      next_command: referee.next_command.map(command_json),
      game_event_count: referee.game_events.len(),
      game_event_proposal_count: referee.game_event_proposals.len(),
      current_action_time_remaining: referee.current_action_time_remaining,
      status_message: referee.status_message.as_deref(),
    }
  }
}

fn match_type_json(value: i32) -> EnumJson {
  EnumJson {
    value,
    name: MatchType::try_from(value)
      .ok()
      .map(|match_type| match_type.as_str_name()),
  }
}

fn stage_json(value: i32) -> EnumJson {
  EnumJson {
    value,
    name: referee::Stage::try_from(value)
      .ok()
      .map(|stage| stage.as_str_name()),
  }
}

fn command_json(value: i32) -> EnumJson {
  EnumJson {
    value,
    name: referee::Command::try_from(value)
      .ok()
      .map(|command| command.as_str_name()),
  }
}

fn hull_color_json(value: i32) -> EnumJson {
  EnumJson {
    value,
    name: HullColor::try_from(value)
      .ok()
      .map(|hull_color| hull_color.as_str_name()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::proto::{HullColor, referee};
  use serde_json::Value;

  #[test]
  fn encodes_interface_data_as_frontend_json() {
    let message = InterfaceDataWire {
      robots: std::array::from_fn(|index| {
        if index == 0 {
          Some(Robot {
            robot_id: 0,
            team: 1,
            pos: crate::vec::types::Vec2::new(100.0, -50.0),
            vel: None,
            orientation: 1.5,
            angular_vel: Some(0.2),
            visibility: 255,
          })
        } else {
          None
        }
      }),
      ball: Some(Ball {
        pos: crate::vec::types::Vec2::new(1.0, 2.0),
        vel: None,
      }),
      game_controller_data: Some(Referee {
        source_identifier: Some("gc".to_string()),
        match_type: Some(MatchType::Friendly as i32),
        packet_timestamp: 10,
        stage: referee::Stage::NormalFirstHalf as i32,
        stage_time_left: Some(30),
        command: referee::Command::Stop as i32,
        command_counter: 2,
        command_timestamp: 20,
        yellow: referee::TeamInfo {
          name: "Yellow".to_string(),
          score: 1,
          red_cards: 0,
          yellow_card_times: vec![100],
          yellow_cards: 1,
          timeouts: 4,
          timeout_time: 5,
          goalkeeper: 0,
          foul_counter: Some(3),
          ball_placement_failures: None,
          can_place_ball: Some(true),
          max_allowed_bots: Some(6),
          bot_substitution_intent: Some(false),
          ball_placement_failures_reached: Some(false),
          bot_substitution_allowed: Some(true),
          bot_substitutions_left: Some(2),
          bot_substitution_time_left: Some(99),
          hull_color: Some(HullColor::Light as i32),
        },
        blue: referee::TeamInfo {
          name: "Blue".to_string(),
          score: 2,
          red_cards: 0,
          yellow_card_times: vec![],
          yellow_cards: 0,
          timeouts: 4,
          timeout_time: 5,
          goalkeeper: 1,
          foul_counter: None,
          ball_placement_failures: None,
          can_place_ball: None,
          max_allowed_bots: None,
          bot_substitution_intent: None,
          ball_placement_failures_reached: None,
          bot_substitution_allowed: None,
          bot_substitutions_left: None,
          bot_substitution_time_left: None,
          hull_color: None,
        },
        designated_position: Some(referee::Point { x: 1.0, y: 2.0 }),
        blue_team_on_positive_half: Some(false),
        next_command: Some(referee::Command::ForceStart as i32),
        game_events: vec![],
        game_event_proposals: vec![],
        current_action_time_remaining: Some(123),
        status_message: Some("running".to_string()),
      }),
      robot_telemetry_wire: std::array::from_fn(|index| {
        if index == 0 {
          Some(RobotTelemetryWire {
            robot_id: 0,
            status: 1,
            seq_seen: 0x0102_0304,
            vx_mmps: -1,
            vy_mmps: 2,
            orientation: 3,
            battery_mv: 24,
            current: 5,
            capacitor_v: 6,
            flags: 0x0102,
          })
        } else {
          None
        }
      }),
      robot_debug_wire: std::array::from_fn(|index| {
        if index == 0 {
          Some(RobotDebugWire {
            robot_id: 0,
            motor_current: [1, 2, 3, 4, 5],
            motor_rotations: [258, 2, 3, 4, 5],
            motor_encoder_rotations: [6, 7, 8, 9],
            temps: [20; 12],
            g_forces: 0,
          })
        } else {
          None
        }
      }),
      robot_sensor_wire: std::array::from_fn(|index| {
        if index == 0 {
          Some(RobotSensorWire {
            robot_id: 0,
            seq: 7,
            ball_angle: -10,
            ball_dist: 11,
            lidar_dist: [42; 450],
          })
        } else {
          None
        }
      }),
    };

    let encoded = message.encode().unwrap();
    let json: Value = serde_json::from_slice(&encoded).unwrap();

    assert_eq!(json["robots"][0]["pos"]["x"], 100.0);
    assert_eq!(json["game_controller_data"]["command"]["name"], "STOP");
    assert_eq!(
      json["game_controller_data"]["yellow"]["hull_color"]["name"],
      "HULL_COLOR_LIGHT"
    );
    assert_eq!(json["robot_telemetry_wire"][0]["seq_seen"], 0x0102_0304);
    assert_eq!(json["robot_debug_wire"][0]["motor_rotations"][0], 258);
    assert_eq!(json["robot_sensor_wire"][0]["lidar_dist"][0], 42);
  }
}
