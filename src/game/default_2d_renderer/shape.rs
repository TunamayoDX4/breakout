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

pub struct Dot {
	vertex_buffer: [Vertex; 1], 
}
impl Dot {
    pub fn rendering(
        self, 
        wgpu_ctx: &crate::gfx::WGContext, 
        view: &wgpu::TextureView, 
        renderer: &DotRenderer, 
    ) {
        let buffer = wgpu_ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertex_buffer),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let mut enc = wgpu_ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("dot renderer command encoder"),
            });
        {
            let mut render_pass = enc.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("dot renderer render pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load, 
                                store: true, 
                            }, 
                        }
                    )],
                    depth_stencil_attachment: None,
                }
            );
            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_vertex_buffer(
                0, 
                buffer.slice(..)
            );
            render_pass.draw(0..1, 0..1)
        }

        wgpu_ctx.queue.submit(std::iter::once(enc.finish()));
    }
}

pub struct DotRenderer {
	pipeline: wgpu::RenderPipeline, 
}
impl DotRenderer {
}

pub struct Line {
	vertex_buffer: [Vertex; 2], 
}
impl Line {
    pub fn rendering(
        self, 
        wgpu_ctx: &crate::gfx::WGContext, 
        view: &wgpu::TextureView, 
        renderer: &LineRenderer, 
    ) {
        let buffer = wgpu_ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertex_buffer),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let mut enc = wgpu_ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("line renderer command encoder"),
            });
        {
            let mut render_pass = enc.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("line renderer render pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load, 
                                store: true, 
                            }, 
                        }
                    )],
                    depth_stencil_attachment: None,
                }
            );
            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_vertex_buffer(
                0, 
                buffer.slice(..)
            );
            render_pass.draw(0..2, 0..1)
        }

        wgpu_ctx.queue.submit(std::iter::once(enc.finish()));
    }
}

pub struct LineRenderer {
	pipeline: wgpu::RenderPipeline, 
}
impl LineRenderer {
}

pub struct Triangle {
	vertex_buffer: [Vertex; 3], 
}
impl Triangle {
    pub fn rendering(
        self, 
        wgpu_ctx: &crate::gfx::WGContext, 
        view: &wgpu::TextureView, 
        renderer: &TriangleRenderer, 
    ) {
        let buffer = wgpu_ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertex_buffer),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let mut enc = wgpu_ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("triangle renderer command encoder"),
            });
        {
            let mut render_pass = enc.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("triangle renderer render pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load, 
                                store: true, 
                            }, 
                        }
                    )],
                    depth_stencil_attachment: None,
                }
            );
            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_vertex_buffer(
                0, 
                buffer.slice(..)
            );
            render_pass.draw(0..3, 0..1)
        }

        wgpu_ctx.queue.submit(std::iter::once(enc.finish()));
    }
}

pub struct TriangleRenderer {
	pipeline: wgpu::RenderPipeline, 
}
impl TriangleRenderer {
}