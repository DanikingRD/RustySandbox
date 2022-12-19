use egui_wgpu_backend::RenderPass;
use tracing::{info, span, Level};
use winit::event::{Event, KeyboardInput, WindowEvent};

use crate::{
    egui_instance::EguiInstance, error::RendererError, renderer::Renderer, window::Window,
};

pub struct Client {
    pub window: Window,
    pub renderer: Renderer,
    pub gui: EguiInstance,
}

impl Client {
    pub fn init(window: Window, renderer: Renderer) -> Self {
        // We use the egui_wgpu_backend crate as the render backend.
        let egui_renderpass = RenderPass::new(&renderer.device, renderer.surface_config.format, 1);
        let gui = crate::egui_instance::EguiInstance::new(egui_renderpass, &window.winit());

        Self {
            window,
            renderer,
            gui,
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        let span = span!(Level::INFO, "update");
        let _guard = span.enter();
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let w = self.window.resolution().x as f32;
                let h = self.window.resolution().y as f32;
                self.renderer.camera.on_update(keycode);
                self.renderer
                    .camera_projection
                    .set_mvp_from_mat(self.renderer.camera.build_mvp(h, w));
                self.renderer.camera_buffer.update(
                    &self.renderer.queue,
                    &[self.renderer.camera_projection],
                    0,
                );
            }
            _ => (),
        }
    }

    pub fn render(&mut self) -> Result<(), RendererError> {
        let mut encoder =
            self.renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Encoder: Frame Main"),
                });

        let texture = self.renderer.start_frame(&mut encoder);

        self.gui.draw(
            &mut self.renderer,
            self.window.winit().scale_factor() as f32,
            &mut encoder,
            &texture,
        );
        self.renderer
            .queue
            .submit(std::iter::once(encoder.finish()));
        texture.present();
        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
    pub fn window_id(&self) -> winit::window::WindowId {
        self.window.winit().id()
    }
}
