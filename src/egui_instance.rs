use std::{iter, time::Instant};

use egui::FontDefinitions;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::{Platform, PlatformDescriptor};
use tracing::{span, Level};

use crate::renderer::RendererBorrow;

pub struct EguiInstance {
    pub platform: Platform,
}

impl EguiInstance {
    pub fn new(window: &winit::window::Window) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        Self { platform }
    }
}

pub fn draw_egui(platform: &mut Platform, mut borrow: RendererBorrow, scale_factor: f32) {
    let span = span!(Level::INFO, "Draw Egui");
    let _guard = span.enter();

    platform.begin_frame();

    let full_output = platform.end_frame(None);

    let paint_jobs = platform.context().tessellate(full_output.shapes);

    // Upload all resources for the GPU.
    let screen_descriptor = ScreenDescriptor {
        physical_width: borrow.surface_config.width,
        physical_height: borrow.surface_config.height,
        scale_factor,
    };

    let tdelta: egui::TexturesDelta = full_output.textures_delta;

    borrow
        .egui_render_pass
        .add_textures(&borrow.device, &borrow.queue, &tdelta)
        .unwrap();

    borrow.egui_render_pass.update_buffers(
        &borrow.device,
        &borrow.queue,
        &paint_jobs,
        &screen_descriptor,
    );

    // borrow
    //     .egui_render_pass
    //     .execute(
    //         &mut borrow.encoder,
    //         &borrow.view,
    //         &paint_jobs,
    //         &screen_descriptor,
    //         Some(wgpu::Color::BLACK),
    //     )
    //     .unwrap();

    // borrow.queue.submit(iter::once(borrow.encoder.finish()));
    // borrow.surface_texture.present();

    borrow.egui_render_pass.remove_textures(tdelta).unwrap();

    drop(_guard);
}
