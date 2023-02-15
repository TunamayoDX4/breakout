use super::comm_renderer;

pub enum D2DRendererComm {
	Dot, 
	Line, 
	Triangle, 
}
impl comm_renderer::RenderCommand for D2DRendererComm {
    type Ctx = D2DRendererCtx;

    fn rendering(
        self, 
        ctx: &Self::Ctx, 
        wgpu_ctx: &crate::gfx::WGContext, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
    ) {
        todo!()
    }
}

pub struct Dot {
	vertex_buffer: wgpu::Buffer, 
}

pub struct DotRenderer {
	pipeline: wgpu::RenderPipeline, 
}

pub struct Line {
	vertex_buffer: wgpu::Buffer, 
}

pub struct LineRenderer {
	pipeline: wgpu::RenderPipeline, 
}

pub struct Triangle {
	vertex_buffer: wgpu::Buffer, 
}

pub struct TriangleRenderer {
	pipeline: wgpu::RenderPipeline, 
}

pub struct D2DRendererCtx {
	dot: DotRenderer, 
	line: LineRenderer, 
	triangle: TriangleRenderer, 
}
impl comm_renderer::RenderCtx for D2DRendererCtx {
    type Comm = D2DRendererComm;
}