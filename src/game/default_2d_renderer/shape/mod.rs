pub mod dot;
pub mod line;
pub mod triangle;

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	pub pos: VertexPos, 
	pub color: VertexColor, 
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x4, 
        1 => Float32x4, 
    ];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &Self::ATTRIBS, 
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexPos ([f32; 4]);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexColor ([f32; 4]);