use crate::proto::{SslDetectionBall, SslGeometryData, Team, TrackedBall, TrackedFrame};
use crate::vec::types::Vec2;
use serde::{Deserialize, Serialize};

/// Internal use generic robot
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Robot {
  pub robot_id: u8,
  /// 0 = Unknown, 1 = Yellow, 2 = Blue
  pub team: u8,

  pub pos: Vec2<f32>,
  pub vel: Option<Vec2<f32>>,

  pub orientation: f32,
  pub angular_vel: Option<f32>,

  pub visibility: u8,
}

impl Default for Robot {
  fn default() -> Robot {
    Robot {
      robot_id: 0,
      team: 0,
      pos: Vec2::new(0.0, 0.0),
      vel: None,
      orientation: 0.0,
      angular_vel: None,
      visibility: 0,
    }
  }
}

impl Robot {
  /// Takes all the robots and converts them to our own robot type
  #[inline]
  pub fn new_from_tracked(frame: &TrackedFrame) -> (Vec<Robot>, Vec<Robot>) {
    let mut robots_yellow: Vec<Robot> = Vec::with_capacity(16);
    let mut robots_blue: Vec<Robot> = Vec::with_capacity(16);

    for robot in &frame.robots {
      if let Some(id) = robot.robot_id.id {
        let r = Robot {
          robot_id: id as u8,
          team: if let Some(team) = robot.robot_id.team {
            team as u8
          } else {
            0
          },
          pos: Vec2::new_from_ssl_vec2(robot.pos),
          vel: robot.vel.map(Vec2::new_from_ssl_vec2),
          orientation: robot.orientation,
          angular_vel: robot.vel_angular,
          visibility: if let Some(vis) = robot.visibility {
            vis.round() as u8
          } else {
            0
          },
        };
        if let Some(team) = robot.robot_id.team
          && team == Team::Yellow as i32
        {
          robots_yellow.insert(id as usize, r);
        } else if let Some(team) = robot.robot_id.team
          && team == Team::Blue as i32
        {
          robots_blue.insert(id as usize, r);
        } else {
          println!("Found robot with no team: {:?}", robot.robot_id);
        }
      } else {
        println!("Found robot with no id: {:?}", robot.robot_id);
      }
    }

    (robots_yellow, robots_blue)
  }
}

/// Our Ball
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Ball {
  pub pos: Vec2<f32>,
  pub vel: Option<Vec2<f32>>,
}

impl Ball {
  #[inline]
  pub fn new_from_vis(
    raw: &[SslDetectionBall],
    tracked: &[TrackedBall],
    raw_or_tracked: bool,
    test_field: Option<u8>,
  ) -> Option<Ball> {
    match raw_or_tracked {
      false => {
        if raw.is_empty() {
          return None;
        }
        // Raw
        match test_field {
          Some(test_field) => match test_field {
            0 => {
              let balls: Vec<Ball> = raw
                .iter()
                .filter(|ball| ball.x.is_sign_positive())
                .map(|ball| Ball {
                  pos: Vec2::new(ball.x, ball.y),
                  vel: Default::default(),
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            1 => {
              let balls: Vec<Ball> = raw
                .iter()
                .filter(|ball| ball.x.is_sign_negative())
                .map(|ball| Ball {
                  pos: Vec2::new(ball.x, ball.y),
                  vel: Default::default(),
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            _ => {
              println!("Wrong Test Mode: 0 = X+, 1 = X-");
              None
            }
          },
          None => Some(Ball {
            pos: Vec2::new(raw[0].x, raw[0].y),
            vel: Default::default(),
          }),
        }
      }
      true => {
        // Tracked

        if tracked.is_empty() {
          return None;
        }
        match test_field {
          Some(test_field) => match test_field {
            0 => {
              let balls: Vec<Ball> = tracked
                .iter()
                .filter(|ball| ball.pos.x.is_sign_positive())
                .map(|ball| Ball {
                  pos: Vec2::new_from_ssl_vec3(ball.pos),
                  vel: ball.vel.map(Vec2::new_from_ssl_vec3),
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            1 => {
              let balls: Vec<Ball> = tracked
                .iter()
                .filter(|ball| ball.pos.x.is_sign_negative())
                .map(|ball| Ball {
                  pos: Vec2::new_from_ssl_vec3(ball.pos),
                  vel: ball.vel.map(Vec2::new_from_ssl_vec3),
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            _ => {
              println!("Wrong Test Mode: 0 = X+, 1 = X-");
              None
            }
          },
          None => Some(Ball {
            pos: Vec2::new_from_ssl_vec3(tracked[0].pos),
            vel: Some(Vec2::new_from_ssl_vec3(tracked[0].vel.unwrap_or_default())),
          }),
        }
      }
    }
  }
}

/// SSL field dimensions in mm, centred at `(0, 0)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldData {
  /// Y extent between the field lines.
  pub height: f32,
  /// X extent between the field lines.
  pub width: f32,
  /// Additional usable space outside every field line.
  pub runoff_area: f32,

  pub goal_width: f32,
  /// Penalty-area depth along X.
  pub penalty_area_width: f32,
  /// Penalty-area span along Y.
  pub penalty_area_height: f32,
}

impl FieldData {
  #[inline]
  pub fn new_from_vis(geom: SslGeometryData) -> Self {
    Self {
      height: geom.field.field_length as f32,
      width: geom.field.field_width as f32,
      runoff_area: geom.field.boundary_width as f32,
      goal_width: geom.field.goal_width as f32,
      penalty_area_width: if let Some(width) = geom.field.penalty_area_width {
        width as f32
      } else {
        0.0
      },
      penalty_area_height: if let Some(height) = geom.field.penalty_area_depth {
        height as f32
      } else {
        0.0
      },
    }
  }
}
