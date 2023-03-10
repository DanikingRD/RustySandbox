use vek::Vec2;
use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window,
};

use crate::{
    error::Error,
    renderer::{self, Renderer},
};

pub struct Window {
    winit: window::Window,
    resolution: Vec2<u32>,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>, Renderer), Error> {
        let event_loop = EventLoop::new();
        let builder = window::WindowBuilder::new().with_title("Rusty Sandbox");
        let window = builder.build(&event_loop).unwrap();

        let size = window.inner_size();
        let this = Self {
            winit: window,
            resolution: Vec2::new(size.width, size.height),
        };
        let renderer = renderer::Renderer::new(&this)?;

        Ok((this, event_loop, renderer))
    }
    pub fn winit(&self) -> &window::Window {
        &self.winit
    }

    pub fn handle_window_events(
        &mut self,
        event: &event::WindowEvent,
        control_flow: &mut winit::event_loop::ControlFlow,
        renderer: &mut Renderer,
    ) {
        match event {
            event::WindowEvent::CloseRequested => {
                self.on_close();
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => renderer.resize(*size),
            // Not sure when is this even emitted.
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.resize(**new_inner_size)
            }
            WindowEvent::KeyboardInput { input, .. } => {}
            _ => (),
        }
    }
    pub fn resolution(&self) -> &Vec2<u32> {
        &self.resolution
    }
    pub fn on_close(&mut self) {}
}
