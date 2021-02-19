use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::image::{LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::sprite::{sprite_sheet_factory, SpriteSheet};
use crate::render::render_player;

mod render;
mod sprite;

pub struct Player<'r> {
    pub position: Point,
    pub sprite: SpriteSheet<'r>,
    pub frame: u32,
}

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
        position: Point::new(0, 0),
        sprite: sprite_sheet_factory(Rect::new(0, 0, 64, 64), texture),
        frame: 0,
    };
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running;},
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { player.frame +=1;println!("{}",player.frame)},
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { player.frame -=1;println!("{}",player.frame)},
                _ => {}
            }
        }
        //update

        //render
        canvas.set_draw_color(Color::RGB(0, 180, 255));
        canvas.clear();

        render_player(&mut canvas, &player);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
