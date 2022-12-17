use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Zeroable, Clone, Copy, Pod)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];
