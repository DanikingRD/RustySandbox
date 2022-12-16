use wgpu::{RequestDeviceError, SurfaceError};

/// Represents any error that may be triggered by the VoxelEngine.
#[derive(Debug)]
pub enum Error {
    Render(RendererError),
}

#[derive(Debug)]
pub enum RendererError {
    AdapterNotFound,
    RequestDeviceError(wgpu::RequestDeviceError),
    SurfaceError(wgpu::SurfaceError),
}

/// Cast RendererError back to base Error
impl From<RendererError> for Error {
    fn from(error: RendererError) -> Self {
        Self::Render(error)
    }
}
/// Cast WGPU builtin [RequestDeviceError] to [RendererError]
impl From<RequestDeviceError> for RendererError {
    fn from(error: RequestDeviceError) -> Self {
        Self::RequestDeviceError(error)
    }
}

/// Cast WGPU builtin [SurfaceError] to [Renderer Error]
impl From<SurfaceError> for RendererError {
    fn from(error: SurfaceError) -> Self {
        Self::SurfaceError(error)
    }
}
