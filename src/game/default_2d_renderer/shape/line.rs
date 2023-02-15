use wgpu::util::DeviceExt;

pub struct Line {
	vertex_buffer: [super::Vertex; 2], 
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
	pub fn new(wgpu_ctx: &crate::gfx::WGContext) -> Self {
		let shader = wgpu_ctx.device.create_shader_module(
			wgpu::ShaderModuleDescriptor {
				label: Some("Line render shader"),
				source: wgpu::ShaderSource::Wgsl(
					include_str!("standard.wgsl").into()
				),
			}
		);
		let pipeline = wgpu_ctx.device.create_render_pipeline(
			&wgpu::RenderPipelineDescriptor {
				label: Some("Line render pipeline"),
				layout: None,
				vertex: wgpu::VertexState { 
					module: &shader, 
					entry_point: "vs_main", 
					buffers: &[
						super::Vertex::desc(), 
					], 
				},
				fragment: Some(wgpu::FragmentState {
					module: &shader,
					entry_point: "fs_main",
					targets: &[Some(wgpu::ColorTargetState {
						format: wgpu_ctx.config.format,
						blend: Some(wgpu::BlendState::ALPHA_BLENDING),
						write_mask: wgpu::ColorWrites::all(),
					})],
				}), 
				primitive: wgpu::PrimitiveState {
					topology: wgpu::PrimitiveTopology::LineList,
					strip_index_format: None,
					front_face: wgpu::FrontFace::Ccw,
					cull_mode: None,
					unclipped_depth: false,
					polygon_mode: wgpu::PolygonMode::Line,
					conservative: false,
				},
				depth_stencil: None,
				multisample: wgpu::MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				},
				multiview: None,
		});
		Self {
			pipeline, 
		}
	}
}