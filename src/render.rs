use crate::assets::{Assets, Biome, BiomeCatalog, SkyId};
use crate::assets::{CAR_FRAME_HEIGHT, CAR_FRAME_WIDTH};
use crate::graphics::sprite::UvRect;
use crate::graphics::Renderer;
use crate::player::{Player, PLAYER_SCALE};
use crate::road;
use glow::Context;

pub fn horizon_y(screen_height: f32) -> f32 {
    screen_height * 0.4
}

pub unsafe fn draw_scene(gl: &Context, renderer: &Renderer, assets: &Assets, player: &Player, biome: Biome, road_scroll: f32) {
    renderer.clear(gl, 0.0, 0.0, 0.0);
    let theme = BiomeCatalog::theme(biome);
    let horizon = horizon_y(renderer.height);
    draw_sky(gl, renderer, assets, theme.sky);
    road::draw_road(
        gl,
        renderer,
        assets,
        theme.road,
        road_scroll,
        player,
        horizon,
    );
    draw_player(gl, renderer, assets, player);
}

pub unsafe fn draw_sky(gl: &Context, renderer: &Renderer, assets: &Assets, sky: SkyId) {
    let horizon = horizon_y(renderer.height);
    let layer = assets.sky_layer(sky);
    renderer.draw_textured_array_quad(
        gl,
        &assets.skies,
        layer,
        0.0,
        0.0,
        renderer.width,
        horizon,
        UvRect::FULL,
    );
}

pub unsafe fn draw_player(gl: &Context, renderer: &Renderer, assets: &Assets, player: &Player) {
    let frame = player.get_player_frame();
    let w = CAR_FRAME_WIDTH as f32 * PLAYER_SCALE;
    let h = CAR_FRAME_HEIGHT as f32 * PLAYER_SCALE;
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    let left = player.position.x - half_w;
    let top = player.position.y - half_h;
    renderer.draw_textured_array_quad(gl, &assets.car, frame as i32, left, top, w, h, UvRect::FULL);
}
