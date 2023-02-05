pub mod shape;

pub trait DrawCommand {
    fn draw(&self);
}

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