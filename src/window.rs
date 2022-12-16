use winit::{
    dpi::PhysicalSize,
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window,
};

use crate::{
    error::Error,
    renderer::{self, Renderer},
};

pub struct Window {
    raw: window::Window,
    renderer: renderer::Renderer,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>), Error> {
        let event_loop = EventLoop::new();
        let builder = window::WindowBuilder::new().with_title("Rusty Sandbox");
        let window = builder.build(&event_loop).unwrap();

        let renderer = renderer::Renderer::new(&window)?;

        let this = Self {
            raw: window,
            renderer,
        };

        Ok((this, event_loop))
    }
    pub fn raw(&self) -> &window::Window {
        &self.raw
    }

    pub fn handle_window_events(
        &mut self,
        event: event::WindowEvent,
        control_flow: &mut winit::event_loop::ControlFlow,
    ) {
        match event {
            event::WindowEvent::CloseRequested => {
                self.on_close();
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => self.on_resize(size),
            // Not sure when is this even emitted.
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.on_resize(*new_inner_size)
            }
            WindowEvent::KeyboardInput { input, .. } => {}
            _ => (),
        }
    }

    pub fn on_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    pub fn on_close(&mut self) {}

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
