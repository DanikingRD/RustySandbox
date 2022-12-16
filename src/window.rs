
use winit::{window, event_loop::EventLoop};

use crate::error::Error;

pub struct Window {
    window: window::Window,
}

impl Window {
    pub fn new() -> Result<(Self, EventLoop<()>), Error> {
        let event_loop = EventLoop::new();
        let builder = window::WindowBuilder::new()
        .with_title("Rusty Sandbox");
        let window = builder.build(&event_loop).unwrap();
        
        let this = Self {
            window,
        };

        Ok((this, event_loop))
    }
}