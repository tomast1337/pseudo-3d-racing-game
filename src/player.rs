use crate::assets::{CAR_FRAME_HEIGHT, CAR_FRAME_WIDTH};
use crate::directions::{MoveDirection, TurnDirection};
use crate::math::Vec2;
use crate::render::horizon_y;
use crate::road;

pub const PLAYER_SCALE: f32 = 3.0;

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub horizontal_speed: f32,
    pub turn: TurnDirection,
    pub movement: MoveDirection,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Player {
    pub fn new(
        position: Vec2,
        max_speed: f32,
        acceleration: f32,
        horizontal_speed: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> Player {
        Player {
            position,
            speed: 0.0,
            turn: TurnDirection::None,
            movement: MoveDirection::Coast,
            max_speed,
            acceleration,
            horizontal_speed,
            screen_width,
            screen_height,
        }
    }

    fn half_car_width() -> f32 {
        CAR_FRAME_WIDTH as f32 * PLAYER_SCALE * 0.5
    }

    fn lateral_limits(&self) -> (f32, f32) {
        let horizon = horizon_y(self.screen_height);
        let (road_min, road_max) = road::lateral_bounds_at_y(
            self.position.y,
            self.screen_width,
            self.screen_height,
            horizon,
        );
        let half_car = Self::half_car_width();
        let min_x = road_min + half_car;
        let max_x = road_max - half_car;
        if min_x >= max_x {
            let center = self.screen_width * 0.5;
            (center, center)
        } else {
            (min_x, max_x)
        }
    }

    pub fn update_player(&mut self, dt: f32) {
        const BRAKE_RATE: f32 = 240.0;
        const COAST_RATE: f32 = 80.0;

        match self.movement {
            MoveDirection::Forward => {
                if self.speed < self.max_speed {
                    self.speed = (self.speed + self.acceleration * dt).min(self.max_speed);
                }
            }
            MoveDirection::Brake => {
                if self.speed > 0.0 {
                    self.speed = (self.speed - BRAKE_RATE * dt).max(0.0);
                } else {
                    self.turn = TurnDirection::None;
                    self.speed = 0.0;
                }
            }
            MoveDirection::Coast => {
                if self.speed > 0.0 {
                    self.speed = (self.speed - COAST_RATE * dt).max(0.0);
                }
                if self.speed == 0.0 {
                    self.turn = TurnDirection::None;
                }
            }
        }

        let (min_x, max_x) = self.lateral_limits();

        if self.speed > 0.0 {
            match self.turn {
                TurnDirection::Right => {
                    if self.position.x < max_x {
                        self.position.x =
                            (self.position.x + self.horizontal_speed * dt).min(max_x);
                    }
                }
                TurnDirection::Left => {
                    if self.position.x > min_x {
                        self.position.x =
                            (self.position.x - self.horizontal_speed * dt).max(min_x);
                    }
                }
                TurnDirection::None => {}
            }
        }

        self.position.x = self.position.x.clamp(min_x, max_x);
    }

    pub fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
        let horizon = horizon_y(height);
        self.position.y = road::default_car_y(height, horizon);
        let (min_x, max_x) = self.lateral_limits();
        self.position.x = self.position.x.clamp(min_x, max_x);
    }

    /// Car strip: frame 0 = hard right, 3 = straight, 6 = hard left.
    pub fn get_player_frame(&self) -> usize {
        if self.speed < 20.0 {
            match self.turn {
                TurnDirection::Right => 2,
                TurnDirection::None => 3,
                TurnDirection::Left => 4,
            }
        } else if self.speed < 60.0 {
            match self.turn {
                TurnDirection::Right => 1,
                TurnDirection::None => 3,
                TurnDirection::Left => 5,
            }
        } else {
            match self.turn {
                TurnDirection::Right => 0,
                TurnDirection::None => 3,
                TurnDirection::Left => 6,
            }
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {}, {:?})",
            self.movement, self.turn, self.speed, self.position
        )
    }
}
