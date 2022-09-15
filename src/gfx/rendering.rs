//! 描画処理の実装

use wgpu::util::DeviceExt;

use super::model::AsInstance;

impl super::WGContext {
    /// 描画処理
    pub(crate) fn rendering(
        &mut self, 
        instances: &impl AsInstance, 
    ) -> Result<(), wgpu::SurfaceError> {

        // 処理部およびエンコーダの初期化
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut enc = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"), 
            }
        );

        // カメラの更新
        self.camera.update(&self.size);
        self.camera_mat.update(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer, 
            0, 
            bytemuck::cast_slice(&[self.camera_mat])
        );

        // 描画データの処理
        // インスタンスの生インスタンスへの変換
        self.raw_instances.init();
        instances.as_instance(&mut self.raw_instances);

        // インスタンスバッファの更新
        self.instances_buffer = self.device.create_buffer_init(
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
        self.queue.submit(std::iter::once(enc.finish()));
        output.present();
    
        Ok(())
    }
}