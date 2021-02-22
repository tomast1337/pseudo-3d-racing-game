use std::time::{Duration, Instant, SystemTime};

use sdl2::event::Event;
use sdl2::image::{LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::sprite::sprite_sheet_factory;
use crate::render::render_player;
use sdl2::EventPump;
use crate::player::Player;
use crate::directions::{TrunDirection, MoveDirection};
use std::ops::{Sub, Add};

mod render;
mod sprite;
mod player;
mod directions;
//mod keyboard;

static mut RUNNING: bool = true;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");


    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.load_texture("assets/textures/Player.png")?;
    let mut player = Player {
        position: Point::new(0, 150),
        sprite: sprite_sheet_factory(Rect::new(0, 0, 64, 64), texture),
        speed: 0,
        acceleration: 2,
        max_speed: 200,
        turn: TrunDirection::None,
        movement: MoveDirection::Stopped,
        horizontal_speed: 5,
    };

    let mut event_pump = sdl_context.event_pump()?;
    unsafe {
        while RUNNING {
            input_event_handler(&mut event_pump, &mut player);
            //update
            update_player(&mut player);
            //render
            canvas.set_draw_color(Color::RGB(0, 180, 255));
            canvas.clear();

            render_player(&mut canvas, &player);

            canvas.present();
            println!("{}", player);

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    Ok(())
}

fn update_player(player: &mut Player) {
    if MoveDirection::Forward == player.movement {
        if player.speed < player.max_speed {
            player.speed += player.acceleration
        } else {
            player.speed = player.speed
        }
    }

    if MoveDirection::Break == player.movement {
        if player.speed > 0 {
            player.speed -= 1
        } else {
            player.speed = 0
        }
    }
    match player.turn {
        TrunDirection::Right => {
            if player.position.x() <= 350 {
                player.position = player.position.offset(player.horizontal_speed, 0)
            }
        }
        TrunDirection::Left => {
            if player.position.x() >= -350 {
                player.position = player.position.offset(-player.horizontal_speed, 0)
            }
        }
        TrunDirection::None => {}
    }
}

fn input_event_handler(event_pump: &mut EventPump, mut player: &mut Player) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => unsafe { RUNNING = false },

            Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => { player.turn = TrunDirection::Left; }
            Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => { player.turn = TrunDirection::Right; }
            Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => { player.turn = TrunDirection::None; }
            Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => { player.turn = TrunDirection::None; }

            Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => { player.movement = MoveDirection::Forward; }
            Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => { player.movement = MoveDirection::Break; }
            Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => { player.movement = MoveDirection::Break; }
            Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => { player.movement = MoveDirection::Break; }

            _ => {}
        }
    }
}