use tetra::graphics::{Texture, Rectangle};
use tetra::math::Vec2;

pub struct SpriteSheet{
    pub frames:Vec<Rectangle>,
    pub texture: Texture,
}
impl SpriteSheet {
    pub fn new(tile_size:Vec2<i32>,texture:Texture) -> SpriteSheet{
        if texture.width() % tile_size.x!=0 && texture.height() % tile_size.y!=0 { panic!("The tiles wont fit") }

        let mut frames:Vec<Rectangle> = Vec::new();
        for i in (0..texture.height()).step_by(tile_size.y as usize){
            for j in (0..texture.width()).step_by(tile_size.x as usize){
                frames.push(Rectangle::new(j as f32,i as f32,tile_size.x as f32,tile_size.y as f32))
            }
        }
        SpriteSheet{
            frames,
            texture,
        }
    }
}