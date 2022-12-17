use egui_wgpu_backend::RenderPass;
use tracing::info;
use winit::dpi::PhysicalSize;

use crate::error::RendererError;
/// The `Renderer` is the SandBox's rendering system.
/// It can interact with the GPU.  
pub struct Renderer {
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// Content of the inner window, excluding the title bar and borders.
    dimensions: PhysicalSize<u32>,
    pub egui_renderpass: egui_wgpu_backend::RenderPass,
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> Result<Self, RendererError> {
        let backend = wgpu::Backends::all();
        let instance = wgpu::Instance::new(backend);

        // This is unsafe because the window handle must be valid, if you find a way to
        // have an invalid winit::Window then you have bigger issues
        let surface = unsafe { instance.create_surface(&window) };

        // Collect and Log adapters
        let adapters = instance
            .enumerate_adapters(backend)
            .enumerate()
            .collect::<Vec<_>>();

        adapters.iter().for_each(|(index, entry)| {
            let info = entry.get_info();
            info!(?info, "graphics device #{}", index);
        });

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .ok_or(RendererError::AdapterNotFound)?;

        let info = adapter.get_info();
        info!(?info, "Selected graphics device");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))?;

        let dimensions = window.inner_size();

        let format = surface.get_supported_formats(&adapter)[0];
        let surface_cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: dimensions.width,
            height: dimensions.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_cfg);

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_renderpass = RenderPass::new(&device, format, 1);

        let renderer = Self {
            surface,
            device,
            queue,
            surface_config: surface_cfg,
            dimensions,
            egui_renderpass,
        };
        Ok(renderer)
    }

    pub fn start_render(&mut self) -> Result<RendererBorrow, RendererError> {
        let texture = self.surface.get_current_texture()?;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Enconder"),
            });
        let texture_view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        // Submit work for this frame
        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
        let borrow = RendererBorrow {
            surface: &self.surface,
            surface_config: &self.surface_config,
            device: &self.device,
            queue: &self.queue,
            egui_render_pass: &mut self.egui_renderpass,
            
        };
        Ok(borrow)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.dimensions = new_size;
        // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
        // See: https://github.com/rust-windowing/winit/issues/208
        // This solves an issue where the app would panic when minimizing on Windows.
        self.surface_config.width = self.dimensions.width.max(1);
        self.surface_config.height = self.dimensions.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}


pub struct RendererBorrow<'a>  {
    pub surface: &'a wgpu::Surface,
    pub surface_config: &'a wgpu::SurfaceConfiguration,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub egui_render_pass: &'a mut egui_wgpu_backend::RenderPass,
}