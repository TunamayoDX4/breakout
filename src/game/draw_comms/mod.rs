/// コマンド用のレンダラのためのコンテキスト
pub trait RenderCtx: Send + Sync {
    type Comm: RenderCommand<Ctx = Self>;
}

/// コマンド用のレンダラのためのコマンド
pub trait RenderCommand: Sized + Send + Sync {
    type Ctx: RenderCtx<Comm = Self>;
    fn rendering(
        self, 
        ctx: &Self::Ctx, 
        wgpu_ctx: &crate::gfx::WGContext, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
    );
}

/// コマンド用のレンダラのためのコマンド・キュー
pub struct CommandRendererQueue<RC: RenderCommand> (Vec<Option<RC>>);
impl<RC: RenderCommand> CommandRendererQueue<RC> {
    pub fn push<T: TryInto<RC>>(&mut self, comm: T) -> Result<(), T::Error> {
        self.0.push(Some(comm.try_into()?));
        Ok(())
    }
    pub fn rendering(
        &mut self, 
        ctx: &RC::Ctx, 
        wgpu_ctx: &crate::gfx::WGContext, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
    ) {
        self.0.iter_mut()
            .filter_map(|comm| comm.take())
            .for_each(|comm| comm.rendering(
                ctx, 
                wgpu_ctx, 
                output, 
                view, 
            ));
        self.0.clear();
    }
}

/// コマンド用のレンダラ
pub struct CommandRenderer<R: RenderCtx> {
    queue: CommandRendererQueue<R::Comm>, 
    ctx: R, 
}
impl<R: RenderCtx> CommandRenderer<R> {
    pub fn rendering(
        &mut self, 
        wgpu_ctx: &crate::gfx::WGContext, 
    ) -> Result<(), wgpu::SurfaceError> {
        let output = wgpu_ctx.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );
        self.queue.rendering(&self.ctx, wgpu_ctx, &output, &view);
        output.present();
        Ok(())
    }
}