use vek::{Mat4, Vec2, Vec3};

pub struct Camera {
    fov: f32,
    pos: Vec3<f32>,
    translation: Vec3<f32>,
    scale: Vec3<f32>,
    rotation: Vec3<f32>,
}
impl Camera {
    pub fn new(
        fov: f32,
        translation: Vec3<f32>,
        scale: Vec3<f32>,
        rotation_deg: Vec3<f32>,
    ) -> Self {
        Self {
            fov,
            pos: Vec3::zero(),
            translation,
            scale,
            rotation: Vec3::new(
                rotation_deg.x.to_radians(),
                rotation_deg.y.to_radians(),
                rotation_deg.z.to_radians(),
            ),
        }
    }
    pub fn create_projection(&self, size: &Vec3<f32>) -> Mat4<f32> {
        let mat4x4: Mat4<f32> = Mat4::translation_3d(self.translation)
            .scaled_3d(self.scale)
            .rotated_x(self.rotation.x)
            .rotated_y(self.rotation.y);
        return mat4x4;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraProjection {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraProjection {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::identity().into_col_arrays(),
        }
    }
    pub fn new_with_data(data: [[f32; 4]; 4]) -> Self {
        Self { view_proj: data }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, size: &Vec3<f32>) {
        self.view_proj = camera.create_projection(size).into_col_arrays();
    }
}
