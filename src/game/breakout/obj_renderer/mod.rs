//! メインレンダラー

use wgpu::{RenderPipeline, Buffer, BindGroup, util::DeviceExt};

/// モデルの実装
pub mod model;

/// カメラの実装
pub mod camera;

pub struct BreakOutRenderer {
    render_pipeline: RenderPipeline, 
    vertex_buffer: Buffer, 
    index_buffer: Buffer, 
    num_indices: u32, 
    raw_instances: model::RawInstArray, 
    instances_buffer: Buffer, 
    camera: camera::Camera, 
    camera_mat: camera::CameraMat, 
    camera_buffer: Buffer, 
    camera_bg: BindGroup, 
}
impl crate::gfx::WGRenderer for BreakOutRenderer {
    fn rendering(
        &mut self, 
        _output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        ctx: &crate::gfx::WGContext, 
    ) {
        let mut enc = ctx.device.create_command_encoder (
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder")
            }
        );
        self.camera.update(&ctx.size);
        self.camera_mat.update(&self.camera);
        ctx.queue.write_buffer(
            &self.camera_buffer, 
            0, 
            bytemuck::cast_slice(&[self.camera_mat])
        );

        self.instances_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance buffer"),
                contents: bytemuck::cast_slice(self.raw_instances.get()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        {
            // レンダーパスの初期化
            let mut render_pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            // パイプラインをセット
            render_pass.set_pipeline(&self.render_pipeline);

            // カメラのユニフォームバッファをセット
            render_pass.set_bind_group(
                0, 
                &self.camera_bg, 
                &[]
            );

            // バーテックスバッファをセット
            render_pass.set_vertex_buffer(
                0, 
                self.vertex_buffer.slice(..)
            );

            // インスタンスバッファをセット
            render_pass.set_vertex_buffer(
                1, 
                self.instances_buffer.slice(..)
            );


            // インデックスバッファをセット
            render_pass.set_index_buffer(
                self.index_buffer.slice(..), 
                wgpu::IndexFormat::Uint16
            );

            // インデックスを利用した描画
            render_pass.draw_indexed(
                0..self.num_indices, 
                0, 
                0..self.raw_instances.len() as _
            );
        }

        ctx.queue.submit(std::iter::once(enc.finish()));
    }
}
impl BreakOutRenderer {
    pub fn new(
        ctx: &crate::gfx::WGContext, 
    ) -> anyhow::Result<Self> {
        let vertex_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(model::VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let index_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index buffer"),
                contents: bytemuck::cast_slice(model::INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = model::INDICES.len() as u32;
        let raw_instances = model::RawInstArray::new();
        let instances_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instances buffer"),
                contents: bytemuck::cast_slice(raw_instances.get()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let camera = camera::Camera {
            pos: [0., 0.].into(),
            view_port: [ctx.size.width as f32, ctx.size.height as f32].into(),
            angle: 0.,
            scale: 1.,
        };
        let mut camera_mat = camera::CameraMat::new();
        camera_mat.update(&camera);

        let camera_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[camera_mat]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let camera_bg_layout = ctx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera bindgroup Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
            }
        );
        let camera_bg = ctx.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Camera bindgroup"),
                layout: &&camera_bg_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ],
            }
        );

        let shader = ctx.device.create_shader_module(
            wgpu::include_wgsl!("../../../../shader/main.wgsl")
        );
        let render_pipeline_layout = ctx.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[
                    &camera_bg_layout, 
                ],
                push_constant_ranges: &[],
            }
        );
        let render_pipeline = ctx.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        model::Vertex::desc(), 
                        model::InstanceRaw::desc(), 
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: ctx.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );
        Ok(Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            raw_instances,
            instances_buffer,
            camera,
            camera_mat, 
            camera_buffer,
            camera_bg,
        })
    }
    pub fn update(&mut self, instances: &impl model::AsInstance) {
        self.raw_instances.init();
        instances.as_instance(&mut self.raw_instances)
    }
}