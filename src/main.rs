mod sprite;
mod player;
mod directions;

use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::{Context, ContextBuilder, State, input};
use tetra::input::Key;
use tetra::math::Vec2;
use crate::sprite::SpriteSheet;
use crate::player::Player;
use crate::directions::{MoveDirection, TrunDirection};


struct GameState {
    player: Player,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let (width, height) = (1280.0, 720.0);

        Ok(GameState {
            player: Player::new(SpriteSheet::new(Vec2::new(64, 64), Texture::new(ctx, "./assets/textures/Player.png")?), Vec2::new(width / 2.0, height / 1.3), 200.0, 6.0, 4.0)
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::P) { println!("{}", self.player) }

        if input::is_key_down(ctx, Key::W) {
            self.player.movement = MoveDirection::Forward
        }
        if input::is_key_down(ctx, Key::S) ||(input::is_key_up(ctx, Key::W) && input::is_key_up(ctx, Key::D)) {
            self.player.movement = MoveDirection::Break
        }

        if self.player.speed > 0.0{
            if input::is_key_up(ctx, Key::A) && input::is_key_up(ctx, Key::D){
                self.player.turn = TrunDirection::None
            }
            if input::is_key_down(ctx, Key::A) {
                self.player.turn = TrunDirection::Left
            }
            if input::is_key_down(ctx, Key::D) {
                self.player.turn = TrunDirection::Right
            }
        }

        self.player.update_player();

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        render_player(&self.player, ctx);

        Ok(())
    }
}

fn render_player(player: &Player, ctx: &mut Context) {
    player.sprite_sheet.texture.draw_region(
        ctx,
        player.sprite_sheet.frames[player.get_player_frame()],
        DrawParams::new().position(player.position).scale(Vec2::new(3.0, 3.0)).origin(Vec2::new(32.0, 32.0)),
    );
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}