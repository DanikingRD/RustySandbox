use vek::{Mat4, Vec2};

pub struct Camera {
    fov: f32
}
impl Camera {
    pub fn new(fov: f32) -> Self {
        Self {
            fov,
        }
    }
    pub fn create_projection(&self, size: Vec2<f32>) -> Mat4<f32> {
        let proj: Mat4<f32> = Mat4::perspective_fov_lh_zo(self.fov, size.x, size.y, 1.0, 100.0);
        return proj;
    }
}


// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub  fn new() -> Self {
    
        Self {
            view_proj: Mat4::identity().into_row_arrays()
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, size: Vec2<f32>) {
        self.view_proj = camera.create_projection(size).into_row_arrays();
    }
}

 

 