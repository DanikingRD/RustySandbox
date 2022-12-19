use tracing::info;
use vek::{FrustumPlanes, Mat4, Vec2, Vec3};
use wgpu::{util::DeviceExt, BufferUsages, CommandEncoder, SurfaceTexture, TextureUsages};
use winit::dpi::PhysicalSize;

use crate::{
    buffer::Buffer,
    camera::{Camera, CameraBufferData},
    error::RendererError,
    vertex::{Vertex, INDICES, VERTICES},
    window::Window,
};
/// The `Renderer` is the SandBox's rendering system.
/// It can interact with the GPU.  
pub struct Renderer {
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub vertex_buffer: Buffer<Vertex>,
    pub camera_buffer: Buffer<CameraBufferData>,
    pub resolution: Vec2<u32>,
    index_buffer: Buffer<u16>,
    pipeline: wgpu::RenderPipeline,
    clear_color: wgpu::Color,
    pub camera_projection: CameraBufferData,
    camera_bind_group: wgpu::BindGroup,
    pub camera: Camera,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, RendererError> {
        let backend = wgpu::Backends::all();
        let instance = wgpu::Instance::new(backend);

        // This is unsafe because the window handle must be valid, if you find a way to
        // have an invalid winit::Window then you have bigger issues
        let surface = unsafe { instance.create_surface(&window.winit()) };

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
        let dimensions = window.resolution();
        let format = surface.get_supported_formats(&adapter)[0];
        let surface_cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: dimensions.x,
            height: dimensions.y,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_cfg);

        let camera_pos = Vec3::new(0.0, 0.0, -3.0);
        let target = Vec3::zero();
        let camera = Camera::new(camera_pos, target);
        let mut camera_buffer_data = CameraBufferData::new();
        camera_buffer_data
            .set_mvp_from_mat(camera.build_mvp(dimensions.x as f32, dimensions.y as f32));
        let camera_buffer = Buffer::new(
            &device,
            &[camera_buffer_data],
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout Descriptor"),
            bind_group_layouts: &[&camera_bind_group_layout],
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
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
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
        let vertex_buffer = Buffer::new(
            &device,
            VERTICES,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        );
        let index_buffer = Buffer::new(&device, INDICES, BufferUsages::INDEX);
        // let instance_buffer = Buffer::instance(&device, &[instance_data]);
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.data().as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let renderer = Self {
            surface,
            device,
            queue,
            surface_config: surface_cfg,
            resolution: *dimensions,
            pipeline,
            vertex_buffer,
            clear_color: wgpu::Color {
                r: 0.2,
                g: 0.6,
                b: 0.5,
                a: 1.0,
            },
            index_buffer,
            camera_buffer,
            camera_projection: camera_buffer_data,
            camera,
            camera_bind_group,
        };
        Ok(renderer)
    }

    pub fn start_frame(&mut self, encoder: &mut CommandEncoder) -> SurfaceTexture {
        let texture = match self.surface.get_current_texture() {
            Ok(tex) => tex,
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!()
            }
        };
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
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.data().slice(..));
            render_pass.set_index_buffer(
                self.index_buffer.data().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.index_buffer.len() as u32, 0, 0..1);
            // render_pass.draw(0..self.vertices.len() as u32, 0..1);
        }
        return texture;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.resolution = Vec2::new(new_size.width, new_size.height);
        // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
        // See: https://github.com/rust-windowing/winit/issues/208
        // This solves an issue where the app would panic when minimizing on Windows.
        self.surface_config.width = self.resolution.x.max(1);
        self.surface_config.height = self.resolution.y.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}
