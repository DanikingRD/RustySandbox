use crate::window::Window;

pub struct Client {
    window: Window,
}

impl Client {
    pub fn init(window: Window) -> Self {
        Self { window }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
    pub fn window_id(&self) -> winit::window::WindowId {
        self.window.raw().id()
    }
}
