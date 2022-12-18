use vek::{Mat4, Vec3};


pub const DEFAULT_VERTICAL_FOV: f32 = 45.0;

/// Fly style camera that allows to freely move around in a 3D scene. 
pub struct Camera {
    /// Field Of View in radians
    pub fov: f32,
    pub eye: Vec3<f32>,
    pub target: Vec3<f32>

}
impl Camera {
    pub fn new(
        eye: Vec3<f32>,
        target: Vec3<f32>
    ) -> Self {
        Self {
            fov: DEFAULT_VERTICAL_FOV,
            eye,
            target
        }
    }
    pub fn build_mvp(&self, width: f32, height: f32) -> Mat4<f32> {
        let model = Mat4::rotation_3d(30.0f32.to_radians(), Vec3::unit_x());
        let projection: Mat4<f32> = Mat4::perspective_fov_lh_zo(self.fov.to_radians(), width, height, 0.1, 100.0);

        let camera_z = (self.eye - self.target).normalized();
        let camera_y: Vec3<f32> = Vec3::unit_y();
     //   let camera_x = camera_y.cross(camera_z); // right vector
        let view: Mat4<f32> = Mat4::look_at_lh(self.eye, self.target, camera_y);
        return projection * view * model;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBufferData {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    /// Model View Projection Matrix
    pub mvp: [[f32; 4]; 4],
}

impl CameraBufferData {
    pub fn new() -> Self {
        Self {
            mvp: Mat4::identity().into_col_arrays(),
        }
    }
    pub fn new_with_data(data: [[f32; 4]; 4]) -> Self {
        Self { mvp: data }
    }

    pub fn set_mvp(&mut self, mvp: [[f32; 4]; 4]) {
        self.mvp = mvp;
    }

    pub fn set_mvp_from_mat(&mut self, mat4x4: Mat4<f32>) {
        self.mvp = mat4x4.into_col_arrays()
    }
}
