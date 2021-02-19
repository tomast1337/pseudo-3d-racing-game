use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct SpriteSheet<'r>{
    pub frames:Vec<Rect>,
    pub texture:Texture<'r>,
}
/// Generates a Sprite Sheet, wont work if tiles size are not divisible by texture size
pub fn sprite_sheet_factory(tile_size:Rect, texture:Texture) -> SpriteSheet {
    let (width, height) = (texture.query().width, texture.query().height);
    if width %tile_size.width()!=0 && height %tile_size.height()!=0 { panic!("The tiles wont fit") }

    let mut frames:Vec<Rect> = Vec::new();
    for i  in (0..height as i32).step_by(tile_size.height() as usize){
        for j in (0..width as i32).step_by(tile_size.width() as usize){
            frames.push(Rect::new(j,i,tile_size.width(),tile_size.height()))
        }
    }

    SpriteSheet {frames,texture,}
}