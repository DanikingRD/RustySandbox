use std::ops::AddAssign;

use egui::FontDefinitions;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::{Platform, PlatformDescriptor};
use tracing::{span, Level};
use vek::{Mat4, Vec2, Vec3};
use wgpu::{CommandEncoder, SurfaceTexture};

use crate::{camera::{CameraBufferData, DEFAULT_VERTICAL_FOV}, renderer::Renderer, vertex::Vertex};

pub struct EguiInstance {
    pub platform: Platform,
    pub render_pass: egui_wgpu_backend::RenderPass,
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
                fn update_camera(renderer: &mut Renderer, w: f32, h: f32) {
                    let mvp = renderer.camera.build_mvp(w, h);
                    renderer.camera_projection.set_mvp_from_mat(mvp);
                    renderer.camera_buffer.update(&renderer.queue, &[renderer.camera_projection], 0);
                }
                let w = renderer.resolution.x as f32;
                let h = renderer.resolution.y as f32;
                ui.label("Camera Settings");

                ui.label("FOV");
                let slider = ui.add(egui::Slider::new(&mut renderer.camera.fov, 1.0..=120.0));
                if slider.changed() {
                    update_camera(renderer, w, h);
                }
                ui.label("Camera X");
                let slider = ui.add(egui::Slider::new(&mut renderer.camera.eye.x, 1.0..=100.0));
                if slider.changed() {
                    update_camera(renderer, w, h);
                }
                ui.label("Camera Y");
                let slider = ui.add(egui::Slider::new(&mut renderer.camera.eye.y, 1.0..=100.0));
                if slider.changed() {
                    update_camera(renderer, w, h);
                }
                ui.label("Camera Z");
                let slider = ui.add(egui::Slider::new(&mut renderer.camera.eye.z, 1.0..=100.0));
                if slider.changed() {
                    update_camera(renderer, w, h);
                }
                ui.label("Target X");
                let slider = ui.add(egui::Slider::new(&mut renderer.camera.target.x, 0.0..=1.0));
                if slider.changed() {
                    update_camera(renderer, w, h);
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
