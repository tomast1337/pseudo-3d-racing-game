use crate::assets::{CAR_FRAME_HEIGHT, CAR_FRAME_WIDTH};
use crate::assets::{Assets, Biome, BiomeCatalog, RoadId, SkyId};
use crate::graphics::sprite::UvRect;
use crate::graphics::Renderer;
use crate::player::{Player, PLAYER_SCALE};
use glow::Context;

fn horizon_y(screen_height: f32) -> f32 {
    screen_height * 0.4
}

pub unsafe fn draw_scene(
    gl: &Context,
    renderer: &Renderer,
    assets: &Assets,
    player: &Player,
    biome: Biome,
    road_scroll: f32,
) {
    renderer.clear(gl, 0.0, 0.0, 0.0);
    let theme = BiomeCatalog::theme(biome);
    draw_sky(gl, renderer, assets, theme.sky);
    draw_ground(gl, renderer, assets, theme.road, road_scroll);
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

pub unsafe fn draw_ground(
    gl: &Context,
    renderer: &Renderer,
    assets: &Assets,
    road: RoadId,
    scroll: f32,
) {
    let horizon = horizon_y(renderer.height);
    let ground_h = renderer.height - horizon;
    let layer = assets.road_layer(road);
    let u_offset = scroll.fract();
    let uv = UvRect {
        u0: u_offset,
        v0: 0.0,
        u1: u_offset + 1.0,
        v1: 1.0,
    };
    renderer.draw_textured_array_quad(
        gl,
        &assets.roads,
        layer,
        0.0,
        horizon,
        renderer.width,
        ground_h,
        uv,
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
    renderer.draw_textured_array_quad(
        gl,
        &assets.car,
        frame as i32,
        left,
        top,
        w,
        h,
        UvRect::FULL,
    );
}
