pub struct RendererPart {
	pipeline: wgpu::RenderPipeline, 
	vertex_buffer: wgpu::Buffer, 
	index_buffer: wgpu::Buffer, 
}

pub struct MultiRendererPart {
	instance_buffer: wgpu::Buffer, 
}

pub struct TexturedRendererPart {
	bind_group: wgpu::BindGroup, 
}