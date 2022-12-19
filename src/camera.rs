use vek::{Mat4, Vec3};
use winit::event::VirtualKeyCode;

pub const DEFAULT_VERTICAL_FOV: f32 = 45.0;

/// Fly style camera that allows to freely move around in a 3D scene.
pub struct Camera {
    /// Field Of View in radians
    pub fov: f32,
    pub eye: Vec3<f32>,
    pub target: Vec3<f32>,
    pub speed: f32,
    pub up: Vec3<f32>,
}
impl Camera {
    /// Create a new [Camera] with the default parameters.
    pub fn new(eye: Vec3<f32>, target: Vec3<f32>) -> Self {
        Self {
            fov: DEFAULT_VERTICAL_FOV,
            eye,
            target,
            speed: 0.1,
            up: Vec3::unit_y(),
        }
    }
    pub fn build_mvp(&self, width: f32, height: f32) -> Mat4<f32> {
        let model = Mat4::translation_3d(Vec3::new(0.0, 0.0, 0.0));
        let projection: Mat4<f32> =
            Mat4::perspective_fov_lh_zo(self.fov.to_radians(), width, height, 0.1, 100.0);
        let view: Mat4<f32> = Mat4::look_at_lh(self.eye, self.target, self.up);
        return projection * view * model;
    }

    pub fn on_update(&mut self, keycode: &VirtualKeyCode) {
        let forward_vec_normal = (self.target - self.eye).normalized();
        let right_vec_normal = self.up.cross(forward_vec_normal).normalized();
        match keycode {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.eye += forward_vec_normal * self.speed;
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.eye -= forward_vec_normal * self.speed;
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.eye += right_vec_normal * self.speed;
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.eye -= right_vec_normal * self.speed;
            }
            _ => (),
        }
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
