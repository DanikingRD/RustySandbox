use egui_wgpu_backend::RenderPass;
use tracing::info;
use wgpu::{
    util::DeviceExt, BufferUsages, CommandEncoder, SurfaceTexture, TextureView, VertexBufferLayout,
};
use winit::dpi::PhysicalSize;

use crate::{
    error::RendererError,
    vertex::{Vertex, VERTICES},
};
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
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout Descriptor"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shaders/shader.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipelime"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x3,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: surface_cfg.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            // how to interpret our vertices when converting them into triangles.
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });


        let renderer = Self {
            surface,
            device,
            queue,
            surface_config: surface_cfg,
            dimensions,
            egui_renderpass,
            pipeline,
            vertex_buffer,
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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
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
            // encoder,
            // surface_texture: texture,
            view: texture_view,
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

pub struct RendererBorrow<'a> {
    pub surface: &'a wgpu::Surface,
    pub surface_config: &'a wgpu::SurfaceConfiguration,
    //pub surface_texture: SurfaceTexture,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub egui_render_pass: &'a mut egui_wgpu_backend::RenderPass,
  //  pub encoder: CommandEncoder,
    pub view: TextureView,
}
