use crate::assets::Assets;
use crate::graphics::Renderer;
use crate::player::Player;
use glow::Context;

pub const PLAYER_SCALE: f32 = 3.0;

fn horizon_y(screen_height: f32) -> f32 {
    screen_height * 0.4
}

const SKY_COLOR: [f32; 4] = [0.45, 0.72, 0.95, 1.0];
const GROUND_COLOR: [f32; 4] = [0.25, 0.28, 0.22, 1.0];

pub unsafe fn draw_scene(gl: &Context, renderer: &Renderer, assets: &Assets, player: &Player) {
    renderer.clear(gl, 0.0, 0.0, 0.0);
    draw_sky(gl, renderer);
    draw_horizon(gl, renderer);
    draw_player(gl, renderer, assets, player);
}

pub unsafe fn draw_sky(gl: &Context, renderer: &Renderer) {
    let horizon = horizon_y(renderer.height);
    renderer.draw_colored_rect(gl, 0.0, 0.0, renderer.width, horizon, SKY_COLOR);
}

pub unsafe fn draw_horizon(gl: &Context, renderer: &Renderer) {
    let horizon = horizon_y(renderer.height);
    renderer.draw_colored_rect(
        gl,
        0.0,
        horizon,
        renderer.width,
        renderer.height - horizon,
        GROUND_COLOR,
    );
}

pub unsafe fn draw_player(gl: &Context, renderer: &Renderer, assets: &Assets, player: &Player) {
    let frame = player.get_player_frame();
    let uv = assets.player_atlas.frames[frame];
    let size = assets.player_atlas.tile_size as f32 * PLAYER_SCALE;
    renderer.draw_textured_quad(
        gl,
        &assets.player_texture,
        player.position.x,
        player.position.y,
        size,
        size,
        uv,
    );
}
