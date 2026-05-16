use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TexturedVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

/// Road slice vertex: full-screen width with per-depth clip bounds for edge-extend UVs.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RoadVertex {
    pub position: [f32; 2],
    pub v: f32,
    pub clip_lo: f32,
    pub clip_hi: f32,
}
