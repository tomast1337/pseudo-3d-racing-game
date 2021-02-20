use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use crate::sprite::SpriteSheet;
use std::borrow::Borrow;
use crate::Player;

pub fn render_player(canvas: &mut WindowCanvas, player: &Player) -> Result<(), String> {
    render_from_sprite_sheet(canvas,
                             player.position,
                             player.sprite.borrow(),
                             player.get_player_frame())
}

pub fn render_from_sprite_sheet(canvas: &mut WindowCanvas, position: Point, sprite_sheet: &SpriteSheet, frame: u32) -> Result<(), String> {
    render_sprite(canvas,
                  sprite_sheet.texture.borrow(),
                  position,
                  sprite_sheet.frames[frame as usize])
}

/// Renders a section of a texture defined by sprite: Rect
pub fn render_sprite(canvas: &mut WindowCanvas,
                     texture: &Texture,
                     position: Point,
                     sprite: Rect) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
}