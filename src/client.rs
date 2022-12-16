use crate::{renderer::Renderer, window::Window};

pub struct Client {
    window: Window,
    renderer: Renderer,
}

impl Client {
    pub fn init(window: Window, renderer: Renderer) -> Self {
        Self { window, renderer }
    }
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn window_id(&self) -> winit::window::WindowId {
        self.window.raw().id()
    }
}
