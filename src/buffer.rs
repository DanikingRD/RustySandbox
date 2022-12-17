use std::marker::PhantomData;

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// Represents a generic buffer  
pub struct Buffer<T> {
    /// The GPU-accessible buffer
    data: wgpu::Buffer,
    /// The size of the buffer array
    len: usize,
    /// Describe the type of this buffer
    data_type: PhantomData<T>,

}
impl<T: Pod> Buffer<T> {
    /// Buffer constructor.
    pub fn new(device: &wgpu::Device, data: &[T], usage: wgpu::BufferUsages) -> Self {
        let contents = bytemuck::cast_slice(data);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(Self::label()),
            contents,
            usage,
        });
        Self {
            data: buffer,
            len: data.len(),
            data_type: PhantomData,
        }
    }
    /// Returns the GPU-accessible buffer.
    pub fn data(&self) -> &wgpu::Buffer {
        &self.data
    }
    /// Returns the number of elements in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns the debug label of this Buffer
    fn label() -> &'static str {
        //TODO: Possibly find a better name for this
        let type_name = std::any::type_name::<T>();
        type_name
    }
}
