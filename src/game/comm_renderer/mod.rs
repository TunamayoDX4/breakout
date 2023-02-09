pub mod shape;
pub mod renderer_part;

pub trait DrawCommand {
    fn draw(&self);
}

pub enum DrawCommandElement {}

pub enum DrawCommandInstance {
    Element(DrawCommandElement), 
    Group(Vec<Option<DrawCommandElement>>), 
}

pub struct DrawCommandQueue(Vec<Option<DrawCommandInstance>>);

pub enum DrawComm<CC: DrawCommand> {
    Dot (shape::Dot), 
    Line (shape::Line), 
    Polygon (shape::Polygon), 
    Rectangle (shape::Rectangle), 
    Quadangle (shape::Quadangle), 
    Custom(CC), 
}
impl<CC: DrawCommand> DrawComm<CC> {
    pub fn draw(&self) { match self {
        Self::Custom(custom) => custom.draw(), 
        _ => todo!(), 
    }}
}

pub struct DrawCommQueue<CC: DrawCommand> (Vec<Option<DrawComm<CC>>>);
impl<CC: DrawCommand> DrawCommQueue<CC> {
    pub fn draw(&mut self, comm: DrawComm<CC>) {
        self.0.push(Some(comm))
    }
    pub fn rendering(&mut self) {
        for comm in self.0.iter_mut()
            .map(|o| o.take())
            .filter_map(|comm| comm)
        { comm.draw() }
        self.0.clear();
    }
}

pub struct Renderer {
    dot: DotRenderer, 
    line: LineRenderer, 
    triangle_line: TriangleLineRenderer, 
    quadangle_line: QuadangleLineRenderer, 
    triangle: TriangleRenderer, 
    quadangle: QuadangleRenderer, 
}

pub struct RendererParts {
    render_pipeline: wgpu::RenderPipeline, 
    vertex_buffer: wgpu::Buffer, 
    index_buffer: wgpu::Buffer, 
    num_indices: u32, 
}

pub struct DotRenderer(RendererParts);
pub struct LineRenderer(RendererParts);
pub struct TriangleLineRenderer(RendererParts);
pub struct QuadangleLineRenderer(RendererParts);

pub struct TriangleRenderer(RendererParts);
pub struct QuadangleRenderer(RendererParts);

