//! Pseudo-3D road: each row of the 512×128 road texture is a depth slice projected per scanline.
//! Horizontal UVs use shader edge-extend (clamp to u=0 / u=1 outside the road band).

use crate::assets::{Assets, RoadId};
use crate::graphics::vertex::RoadVertex;
use crate::graphics::Renderer;
use crate::player::Player;
use glow::Context;

/// Depth rows in road textures (all `road_*.png` are 512×128).
pub const ROAD_DEPTH_ROWS: f32 = 128.0;
const ROAD_SLICES: usize = ROAD_DEPTH_ROWS as usize;

/// Asphalt band inside the 512px road strip (grass/shoulders outside).
pub const ROAD_TEX_INNER_LO: f32 = 0.12;
pub const ROAD_TEX_INNER_HI: f32 = 0.88;

/// World Z: small = near camera, large = far (toward horizon).
fn depth_to_z(depth: f32) -> f32 {
    const Z_NEAR: f32 = 1.0;
    const Z_FAR: f32 = 22.0;
    let d = depth.clamp(0.0, 1.0);
    Z_NEAR + (1.0 - d) * (Z_FAR - Z_NEAR)
}

/// Map normalized depth [0=far, 1=near] to screen Y (1/z perspective).
fn depth_to_screen_y(depth: f32, horizon: f32, ground_h: f32) -> f32 {
    let z_near = depth_to_z(1.0);
    let z_far = depth_to_z(0.0);
    let z = depth_to_z(depth);
    let inv_z = 1.0 / z;
    let inv_near = 1.0 / z_near;
    let inv_far = 1.0 / z_far;
    let t = (inv_z - inv_far) / (inv_near - inv_far);
    horizon + t.clamp(0.0, 1.0) * ground_h
}

/// Road half-width on screen at this depth (for clip bounds).
pub fn depth_to_half_width(depth: f32, screen_w: f32) -> f32 {
    let z_near = depth_to_z(1.0);
    let z = depth_to_z(depth);
    let scale = (z_near / z).clamp(0.02, 1.0);
    screen_w * 0.5 * scale
}

/// How much of the asphalt band the car may use (centered on screen).
pub const DRIVABLE_ROAD_FRAC: f32 = 0.55;

/// Inverse of [`depth_to_screen_y`]: screen Y below horizon → depth [0=far, 1=near].
pub fn screen_y_to_depth(screen_y: f32, horizon: f32, ground_h: f32) -> f32 {
    if ground_h <= 0.0 {
        return 1.0;
    }
    let t = ((screen_y - horizon) / ground_h).clamp(0.0, 1.0);
    let z_near = depth_to_z(1.0);
    let z_far = depth_to_z(0.0);
    let inv_near = 1.0 / z_near;
    let inv_far = 1.0 / z_far;
    let inv_z = inv_far + t * (inv_near - inv_far);
    let z = 1.0 / inv_z.max(0.001);
    let d = 1.0 - (z - z_near) / (z_far - z_near);
    d.clamp(0.0, 1.0)
}

/// Drivable X range for the car center at a given screen Y (matches road width there).
pub fn lateral_bounds_at_y(screen_y: f32, screen_w: f32, screen_h: f32, horizon: f32) -> (f32, f32) {
    let ground_h = screen_h - horizon;
    let depth = screen_y_to_depth(screen_y, horizon, ground_h);
    let half_road = depth_to_half_width(depth, screen_w);
    let asphalt_frac = ROAD_TEX_INNER_HI - ROAD_TEX_INNER_LO;
    let drivable_half = half_road * asphalt_frac * DRIVABLE_ROAD_FRAC;
    let center = screen_w * 0.5;
    (center - drivable_half, center + drivable_half)
}

/// Default car Y: near the bottom of the road band.
pub fn default_car_y(screen_h: f32, horizon: f32) -> f32 {
    let ground_h = screen_h - horizon;
    horizon + ground_h / 1.5
}

fn screen_clip_bounds(half_w: f32, screen_w: f32, center_x: f32) -> (f32, f32) {
    let lo = ((center_x - half_w) / screen_w).clamp(0.0, 1.0);
    let hi = ((center_x + half_w) / screen_w).clamp(0.0, 1.0);
    (lo, hi)
}

fn push_road_trapezoid(out: &mut Vec<RoadVertex>, screen_w: f32, y_top: f32, y_bottom: f32, clip_lo_top: f32, clip_hi_top: f32, clip_lo_bot: f32, clip_hi_bot: f32, v_top: f32, v_bottom: f32) {
    let left = 0.0;
    let right = screen_w;

    out.extend_from_slice(&[
        RoadVertex {
            position: [left, y_top],
            v: v_top,
            clip_lo: clip_lo_top,
            clip_hi: clip_hi_top,
        },
        RoadVertex {
            position: [right, y_top],
            v: v_top,
            clip_lo: clip_lo_top,
            clip_hi: clip_hi_top,
        },
        RoadVertex {
            position: [right, y_bottom],
            v: v_bottom,
            clip_lo: clip_lo_bot,
            clip_hi: clip_hi_bot,
        },
        RoadVertex {
            position: [left, y_top],
            v: v_top,
            clip_lo: clip_lo_top,
            clip_hi: clip_hi_top,
        },
        RoadVertex {
            position: [right, y_bottom],
            v: v_bottom,
            clip_lo: clip_lo_bot,
            clip_hi: clip_hi_bot,
        },
        RoadVertex {
            position: [left, y_bottom],
            v: v_bottom,
            clip_lo: clip_lo_bot,
            clip_hi: clip_hi_bot,
        },
    ]);
}

pub unsafe fn draw_road(gl: &Context, renderer: &Renderer, assets: &Assets, road: RoadId, scroll: f32, player: &Player, horizon: f32) {
    let screen_w = renderer.width;
    let ground_h = renderer.height - horizon;
    if ground_h <= 0.0 {
        return;
    }

    let layer = assets.road_layer(road);
    let mut vertices = Vec::with_capacity(ROAD_SLICES * 6);

    let player_offset = (player.position.x / screen_w - 0.5) * 0.35;
    let uv_shift = -player_offset;

    for i in 0..ROAD_SLICES {
        let z0 = i as f32 / ROAD_SLICES as f32;
        let z1 = (i + 1) as f32 / ROAD_SLICES as f32;

        let y0 = depth_to_screen_y(z0, horizon, ground_h);
        let y1 = depth_to_screen_y(z1, horizon, ground_h);
        if y1 <= y0 {
            continue;
        }

        let half_w0 = depth_to_half_width(z0, screen_w);
        let half_w1 = depth_to_half_width(z1, screen_w);
        let cx0 = screen_w * 0.5 - player_offset * screen_w * z0;
        let cx1 = screen_w * 0.5 - player_offset * screen_w * z1;
        let (clip_lo0, clip_hi0) = screen_clip_bounds(half_w0, screen_w, cx0);
        let (clip_lo1, clip_hi1) = screen_clip_bounds(half_w1, screen_w, cx1);

        // Negative scroll: driving forward advances texture rows toward the camera.
        let row0 = (z0 * (ROAD_DEPTH_ROWS - 1.0) - scroll * ROAD_DEPTH_ROWS).rem_euclid(ROAD_DEPTH_ROWS);
        let row1 = (z1 * (ROAD_DEPTH_ROWS - 1.0) - scroll * ROAD_DEPTH_ROWS).rem_euclid(ROAD_DEPTH_ROWS);
        let v0 = row0 / ROAD_DEPTH_ROWS;
        let v1 = row1 / ROAD_DEPTH_ROWS;

        push_road_trapezoid(
            &mut vertices,
            screen_w,
            y0,
            y1,
            clip_lo0,
            clip_hi0,
            clip_lo1,
            clip_hi1,
            v0,
            v1,
        );
    }

    renderer.draw_road_mesh(
        gl,
        &assets.roads,
        layer,
        &vertices,
        uv_shift,
        ROAD_TEX_INNER_LO,
        ROAD_TEX_INNER_HI,
    );
}
