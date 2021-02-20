use sdl2::rect::Point;
use crate::sprite::SpriteSheet;
use crate::directions::{TrunDirection, MoveDirection};

pub struct Player<'r> {
    pub position: Point,
    pub sprite: SpriteSheet<'r>,
    pub speed: i32,
    pub max_speed: i32,
    pub acceleration: i32,
    pub turn: TrunDirection,
    pub movement: MoveDirection,
}

impl Player<'_> {
    pub fn get_player_frame(&self) -> u32 {
        if self.speed < 20 {
            match self.turn {
                TrunDirection::Right => { 4 }
                TrunDirection::None => { 3 }
                TrunDirection::Left => { 2 }
            }
        } else {
            if self.speed < 60 {
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

impl std::fmt::Display for Player<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?},{},{:?})", self.movement, self.turn, self.speed,self.position)
    }
}