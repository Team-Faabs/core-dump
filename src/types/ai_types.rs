pub mod sparse;

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::vec::types::{Rect, Vec2};
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
    pub motion_status: MotionStatus
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
    pub motion: Option<MotionCommand>,
    pub kicker: Kicker,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MotionCommand {
    pub target: Target,
    pub heading: HeadingMode,
    pub limits: Option<Limits>,
    pub tolerance: Tolerance,
    pub obstacles: ObstacleFlags,
    pub priority: u8, //For orca, if it needs to move other robots away
    pub deadline: Option<f32>,
    pub id: Id,
}


#[derive(Debug, Clone, Copy, Default)]
pub enum Target {
    Pos(Vec2<f32>),
    Heading { heading: f32 }, //drive to heading not turn to heading
    Velocity {
        vx: f32,
        vy: f32,
    },
    #[default]
    Hold
}

#[derive(Debug, Clone, Copy, Default)]
pub enum HeadingMode {
    Fixed(f32),
    FaceTarget(Vec2<f32>),
    FaceBall,
    FaceRobot(Robot, Team),
    #[default]
    Free,
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub v_max: f32,
    pub a_max: f32,
    pub omega_max: f32,
    pub alpha_max: f32,
    pub jerk: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Tolerance {
    pub pos_mm: f32,
    pub heading_deg: f32,
    pub vel: f32,
}

impl Default for Tolerance {
    fn default() -> Self {
        Self {
            pos_mm: 10.0,
            heading_deg: 2.0,
            vel: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObstacleFlags {
    pub avoid_ball: bool,
    pub raw_movement: bool,
    pub defense_area: bool,
    pub keep_out: Option<Rect<f32>>,
    pub ignore: RobotSelector,
}


impl Default for ObstacleFlags {
    fn default() -> Self {
        Self {
            avoid_ball: true,
            raw_movement: false,
            defense_area: false,
            keep_out: None,
            ignore: RobotSelector::none(),
        }
    }
}


#[derive(Default, Clone, Copy)]
//Bitset for all robots
pub struct RobotSelector {
    pub own: u16,
    pub opp: u16,
}

impl Debug for RobotSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RobotSelector {{ own: {:016b}, opp: {:016b} }}", self.own, self.opp)
    }
}

impl RobotSelector {
    fn new(own: u16, opp: u16) -> Self {
        Self { own, opp }
    }

    fn none() -> Self {
        Self::default()
    }

    fn all_own() -> Self {
        Self::new(!0, 0)
    }

    fn all_opp() -> Self {
        Self::new(0, !0)
    }

    fn all() -> Self {
        Self::new(!0, !0)
    }

    fn set_own(mut self, idx: u8) -> Self {
        self.own |= 1 << idx;
        self
    }

    fn set_opp(mut self, idx: u8) -> Self {
        self.opp |= 1 << idx;
        self
    }

    fn ignore_own(robot: Robot) -> Self {
        Self::none().set_own(robot as u8)
    }

    fn ignore_opp(robot: Robot) -> Self {
        Self::none().set_opp(robot as u8)
    }

    fn from_set(own: &[Robot], opp: &[Robot]) -> Self {
        let mut slf = Self::none();

        for r in own {
            slf = slf.set_own((*r) as u8);
        }

        for r in opp {
            slf = slf.set_opp((*r) as u8);
        }

        slf
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct MotionStatus {
    pub drive: DriveStatus,
    pub heading: HeadingStatus,
    pub id: Id,
}


#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DriveStatus {
    Running { eta: f32, progress: f32, dist: f32 },
    #[default]
    Reached,
    Blocked { progress: f32 },
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HeadingStatus {
    Running { eta: f32, progress: f32, diff: f32 },
    #[default]
    Reached,
    Tracking,
    TrackingBehind(f32),
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


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Id(u32);

impl Default for Id {
    fn default() -> Self {
        Self(Self::next_id())
    }
}

impl Deref for Id {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Id {
    pub const ZERO: Self = Self(0);

    fn get(self) -> u32 {
        self.0
    }

    fn next_id() -> u32 {
        let id = ID.load(Ordering::Relaxed);

        ID.store(id.wrapping_add(1), Ordering::Relaxed);

        id
    }
}

static ID: AtomicU32 = AtomicU32::new(1);
