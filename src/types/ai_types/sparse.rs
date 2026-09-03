use crate::types::ai_types::{Robot, Team};
use crate::vec::types::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct SparseRobot {
  pub id: Robot,
  pub pos: Vec2<i16>,
  pub vel: Vec2<i16>,
  pub heading: u16,
  pub angular_vel: i16,
  pub is_goalie: bool,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SparseBall {
  pub pos: Vec2<i16>,
  pub vel: Vec2<i16>,
  pub stop_pos: Vec2<i16>,
  pub stop_time: f32,
}

pub type SparseRobots = [Option<SparseRobot>; 12];

#[derive(Debug, Copy, Clone, Default)]
pub struct SparseWorld {
  pub own_robots: SparseRobots,
  pub opp_robots: SparseRobots,
  pub ball: SparseBall,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum SparseGameStage {
  Running,
  Stop,
  #[default]
  Halt,
  BallPlacement(Vec2<i16>, Team),
  PrepareKickoff,
  Kickoff,
  FreeKick,
  PenaltyKick(Team),
  ShootOut(Team),
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SparseGameState {
  pub world: SparseWorld,
  pub stage: SparseGameStage,
}
