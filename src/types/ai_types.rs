pub mod sparse;

use crate::vec::types::Vec2;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Default)]
pub struct BallState {
    pub pos: Vec2<f32>,
    pub vel: Vec2<f32>,
    pub stop_pos: Vec2<f32>,
    pub stop_time: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RobotState {
    pub id: u8,
    pub pos: Vec2<f32>,
    pub vel: Vec2<f32>,
    pub heading: f32,
    pub angular_vel: f32,
    pub is_goalie: bool,
}

pub type Robots = [Option<RobotState>; 16];


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    Own,
    Opp,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum GameStage {
    Running,
    Stop,
    #[default]
    Halt,
    BallPlacement(Vec2<f32>, Team),
    PrepareKickoff,
    Kickoff,
    FreeKick,
    PenaltyKick(Team),
    ShootOut(Team),
}

#[derive(Debug, Copy, Clone, Default)]
pub struct World {
    pub own_robots: Robots,
    pub opp_robots: Robots,
    pub ball: BallState,
}


#[derive(Debug, Copy, Clone, Default)]
pub struct GameState {
    pub world: World,
    pub stage: GameStage,
}


#[derive(Debug, Clone, Copy, Default)]
pub enum Intent {
    Goalie,
    PassTo(Robot),
    RecPass,
    KickGoal,
    Block,
    Wall,
    Steal,
    GetBallTurn,
    GetBallBehind,
    SmashBall,
    #[default]
    Hold,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum Robot {
    #[default]
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}


#[derive(Debug, Clone, Copy, Default)]
pub struct RobotCommand {
    pub dribbler: bool,
    pub raw_movement: bool,
    pub avoid_ball_collision: bool,
    pub pos: Option<Pos>,
    pub kicker: Kicker,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pos {
    pub pos: Option<Vec2<f32>>,
    pub face: Option<f32>,
    pub speed: Option<u32>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Kicker {
    #[default]
    None,
    Chip(f32), //dist in mm
    Kick(f32), // dist in mm
}


pub type Commands = [Option<RobotCommand>; 16];

pub trait Ai {
    fn predict(&mut self, state: GameState) -> Commands;

    fn debug(&self) -> String {
        String::new()
    }
}


#[derive(Default)]
pub struct DummyAi;


impl Ai for DummyAi {
    fn predict(&mut self, _state: GameState) -> Commands {
        Commands::default()
    }
}
