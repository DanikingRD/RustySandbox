use winit::{event_loop::EventLoop, window};

use crate::{error::Error, renderer::{self, Renderer}};

pub struct Window {
    window: window::Window,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>, Renderer), Error> {
        let event_loop = EventLoop::new();
        let builder = window::WindowBuilder::new().with_title("Rusty Sandbox");
        let window = builder.build(&event_loop).unwrap();

        let renderer = renderer::Renderer::new(&window)?;

        let this = Self { window };

        Ok((this, event_loop, renderer))
    }
}
