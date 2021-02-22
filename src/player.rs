use crate::sprite::SpriteSheet;
use crate::directions::{TrunDirection, MoveDirection};
use tetra::math::Vec2;

pub struct Player {
    pub sprite_sheet: SpriteSheet,
    pub position: Vec2<f32>,
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub horizontal_speed: f32,
    pub turn: TrunDirection,
    pub movement: MoveDirection,
}

impl Player {
    pub fn new(sprite_sheet: SpriteSheet, position: Vec2<f32>,max_speed: f32,acceleration: f32,horizontal_speed: f32) -> Player {
        Player {
            sprite_sheet,
            position,
            speed: 0.0,
            turn: TrunDirection::None,
            movement: MoveDirection::Break,
            max_speed,
            acceleration,
            horizontal_speed,
        }
    }

    pub fn update_player(&mut self) {
        match self.movement{
            MoveDirection::Forward =>{
                if self.speed < self.max_speed {
                    self.speed += self.acceleration
                } else {
                    self.speed = self.speed
                }
            },
            MoveDirection::Break =>{
                if self.speed > 0.0 {
                    self.speed -= 4.0
                } else {
                    self.turn = TrunDirection::None;
                    self.speed = 0.0
                }
            },
        }

        match self.turn {
            TrunDirection::Right => {
                if self.position.x <= 1216.0 {
                    self.position.x += self.horizontal_speed
                }
            }
            TrunDirection::Left => {
                if self.position.x >= 64.0 {
                    self.position.x -= self.horizontal_speed
                }
            }
            TrunDirection::None => {}
        }
    }

    pub fn get_player_frame(&self) -> usize {
        if self.speed < 20.0 {
            match self.turn {
                TrunDirection::Right => { 4 }
                TrunDirection::None => { 3 }
                TrunDirection::Left => { 2 }
            }
        } else {
            if self.speed < 60.0 {
                match self.turn {
                    TrunDirection::Right => { 5 }
                    TrunDirection::None => { 3 }
                    TrunDirection::Left => { 1 }
                }
            } else {
                match self.turn {
                    TrunDirection::Right => { 6 }
                    TrunDirection::None => { 3 }
                    TrunDirection::Left => { 0 }
                }
            }
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?},{},{:?})", self.movement, self.turn, self.speed, self.position)
    }
}