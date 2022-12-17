use std::{time::Instant, iter};

use egui_wgpu_backend::ScreenDescriptor;
use tracing::{span, Level};

use crate::{egui_instance::EguiInstance, window::Window, renderer::RendererBorrow};

pub struct Client {
    window: Window,
    pub egui: EguiInstance,
}

impl Client {
    pub fn init(window: Window) -> Self {
        let egui = EguiInstance::new(&window.raw());
        Self { window, egui }
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
