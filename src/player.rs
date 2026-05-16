use crate::assets::{CAR_FRAME_HEIGHT, CAR_FRAME_WIDTH};
use crate::directions::{MoveDirection, TurnDirection};
use crate::math::Vec2;

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
}

impl Player {
    pub fn new(
        position: Vec2,
        max_speed: f32,
        acceleration: f32,
        horizontal_speed: f32,
        screen_width: f32,
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
        }
    }

    fn half_car_width() -> f32 {
        CAR_FRAME_WIDTH as f32 * PLAYER_SCALE * 0.5
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

        let half_car = Self::half_car_width();
        let min_x = half_car;
        let max_x = self.screen_width - half_car;

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
    }

    pub fn set_screen_width(&mut self, width: f32) {
        self.screen_width = width;
        let half_car = Self::half_car_width();
        self.position.x = self.position.x.clamp(half_car, width - half_car);
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
