use std::ops::AddAssign;

use egui::FontDefinitions;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::{Platform, PlatformDescriptor};
use tracing::{span, Level};
use vek::{Mat4, Vec2};
use wgpu::{CommandEncoder, SurfaceTexture};

use crate::{camera::CameraProjection, renderer::Renderer, vertex::Vertex};

pub struct EguiInstance {
    pub platform: Platform,
    pub render_pass: egui_wgpu_backend::RenderPass,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: Vec2<f32>,
}

impl EguiInstance {
    pub fn new(render_pass: egui_wgpu_backend::RenderPass, window: &winit::window::Window) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        Self {
            platform,
            render_pass,
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotation: Vec2::zero(),
        }
    }

    pub fn handle_event<T>(&mut self, winit_event: &winit::event::Event<T>) {
        self.platform.handle_event(winit_event);
    }

    pub fn draw(
        &mut self,
        renderer: &mut Renderer,
        scale_factor: f32,
        encoder: &mut CommandEncoder,
        texture: &SurfaceTexture,
    ) {
        let span = span!(Level::INFO, "Draw Egui");
        let _guard = span.enter();
        self.platform.begin_frame();

        egui::Window::new("EGUI Instance")
            .default_size([340.0, 700.0])
            .resizable(true)
            .title_bar(false)
            .show(&self.platform.context(), |ui| {
                ui.label("Scale");
                let slider = ui.add(egui::Slider::new(&mut self.scale, 0.0..=2.0));
                if slider.changed() {
                    let translation = renderer.object_pos;
                    renderer.scale.add_assign(self.scale);
                    let new_matrix: Mat4<f32> =
                        Mat4::translation_3d(translation).scaled_3d(self.scale);
                    let uniform = CameraProjection::new_with_data(new_matrix.into_col_arrays());
                    renderer
                        .camera_buffer
                        .update(&renderer.queue, &[uniform], 0);
                }

                ui.label("Translate X");
                let slider = ui.add(egui::Slider::new(&mut self.x, -1.0..=1.0));
                if slider.changed() {
                    let previous_translation = renderer.object_pos;
                    let translation_x = previous_translation.with_x(self.x).with_y(self.y);
                    let new_matrix: Mat4<f32> =
                        Mat4::translation_3d(translation_x).scaled_3d(self.scale);
                    let uniform = CameraProjection::new_with_data(new_matrix.into_col_arrays());
                    renderer
                        .camera_buffer
                        .update(&renderer.queue, &[uniform], 0);
                }
                ui.label("Translate Y");
                let slider = ui.add(egui::Slider::new(&mut self.y, -1.0..=1.0));
                if slider.changed() {
                    let previous_translation = renderer.object_pos;
                    let translation_y = previous_translation.with_y(self.y).with_x(self.x);
                    let new_matrix: Mat4<f32> =
                        Mat4::translation_3d(translation_y).scaled_3d(self.scale);
                    let uniform = CameraProjection::new_with_data(new_matrix.into_col_arrays());
                    renderer
                        .camera_buffer
                        .update(&renderer.queue, &[uniform], 0);
                }

                ui.label("Rotate X (deg)");
                let slider = ui.add(egui::Slider::new(&mut self.rotation.x, 0.0..=360.0).step_by(0.5));
                if slider.changed() {
                    let translation = renderer.object_pos;
                    renderer.scale.add_assign(self.scale);

                    let new_matrix: Mat4<f32> = Mat4::translation_3d(translation)
                        .scaled_3d(self.scale)
                        .rotated_x(self.rotation.x.to_radians())
                        .rotated_y(self.rotation.y.to_radians());
                    let uniform = CameraProjection::new_with_data(new_matrix.into_col_arrays());

                    renderer
                        .camera_buffer
                        .update(&renderer.queue, &[uniform], 0);
                }
                ui.label("Rotate Y (deg)");
                let slider = ui.add(egui::Slider::new(&mut self.rotation.y, 0.0..=360.0).step_by(0.5));
                if slider.changed() {
                    let translation = renderer.object_pos;
                    renderer.scale.add_assign(self.scale);
                    let new_matrix: Mat4<f32> = Mat4::translation_3d(translation)
                        .scaled_3d(self.scale)
                        .rotated_y(self.rotation.y.to_radians())
                        .rotated_x(self.rotation.x.to_radians());
                    let uniform = CameraProjection::new_with_data(new_matrix.into_col_arrays());
                    renderer
                        .camera_buffer
                        .update(&renderer.queue, &[uniform], 0);
                }
            });

        let full_output = self.platform.end_frame(None);

        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: renderer.surface_config.width,
            physical_height: renderer.surface_config.height,
            scale_factor,
        };

        let tdelta: egui::TexturesDelta = full_output.textures_delta;

        self.render_pass
            .add_textures(&renderer.device, &renderer.queue, &tdelta)
            .unwrap();

        self.render_pass.update_buffers(
            &renderer.device,
            &renderer.queue,
            &paint_jobs,
            &screen_descriptor,
        );

        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // Record all render passes
        self.render_pass
            .execute(encoder, &view, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        self.render_pass
            .remove_textures(tdelta)
            .expect("Failed to remove texture");

        drop(_guard);
    }
}
