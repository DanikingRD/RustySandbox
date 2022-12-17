use std::{time::Instant, iter};

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

pub fn draw_egui(platform: &mut Platform, borrow: RendererBorrow) {
    let start_time = Instant::now();
    let span = span!(Level::INFO, "Draw Egui");
    let _guard = span.enter();
    let mut demo_app = egui_demo_lib::DemoWindows::default();
    platform.update_time(start_time.elapsed().as_secs_f64());

    let output_frame = match borrow.surface.get_current_texture() {
        Ok(frame) => frame,
        Err(_) => todo!(),
    };
    let output_view = output_frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    platform.begin_frame();

    demo_app.ui(&platform.context());

    let full_output = platform.end_frame(None);
    let paint_jobs = platform.context().tessellate(full_output.shapes);
    let mut encoder = borrow
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });
    // Upload all resources for the GPU.
    let screen_descriptor = ScreenDescriptor {
        physical_width: borrow.surface_config.width,
        physical_height: borrow.surface_config.height,
        scale_factor: 100.0,
    };
    let tdelta: egui::TexturesDelta = full_output.textures_delta;
    borrow.egui_render_pass.add_textures(&borrow.device, &borrow.queue, &tdelta).unwrap();

    borrow.egui_render_pass.update_buffers(&borrow.device, &borrow.queue, &paint_jobs, &screen_descriptor);    
    
    borrow.egui_render_pass.execute(&mut encoder, &output_view, &paint_jobs, 
        &screen_descriptor, Some(wgpu::Color::BLACK)).unwrap();
    
        borrow.queue.submit(iter::once(encoder.finish()));
    output_frame.present();

    borrow.egui_render_pass.remove_textures(tdelta).unwrap();
    drop(_guard);
}
