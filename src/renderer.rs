use tracing::info;


use crate::error::{RendererError};

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
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

        let adapter = pollster::block_on(
            instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
        ).ok_or(RendererError::AdapterNotFound)?;

        let info = adapter.get_info();
        info!(?info, "Selected graphics device");

        let (device, queue) = pollster::block_on(
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                },
                None,
            ),
        )?;
        
        let renderer = Self {
            surface,
            device,
            queue,
        };
        
        Ok(renderer)
    }
}
