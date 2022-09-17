//! テキストのエントリ

use wgpu_glyph::{
	Section, Text, Layout, BuiltInLineBreaker
};

/// テキストのオブジェクト
pub mod body;

/// 描画するテキスト
pub struct TextObj {
    pub text: body::TextBody, 
    pub scale: f32, 
    pub color: [f32; 4], 
}
impl<'a> From<&'a mut TextObj> for Text<'a> {
    fn from(value: &'a mut TextObj) -> Self {
        Text::new(<&str>::from(&mut value.text))
            .with_scale(value.scale)
            .with_color(value.color)
    }
}

/// 描画するテキストのエントリー
pub struct TextEntry {
    position: nalgebra::Vector2<f32>, 
    text: Vec<TextObj>, 
    layout: Layout<BuiltInLineBreaker>, 
}
impl TextEntry {
    pub fn new<P>(
        position: P, 
        text: Vec<TextObj>, 
        layout: Layout<BuiltInLineBreaker>, 
    ) -> Self where
        P: Into<nalgebra::Vector2<f32>>
    { Self {
        position: position.into(),
        text,
        layout, 
    } }
    pub fn position(&self) -> &nalgebra::Vector2<f32> { &self.position }
    pub fn position_mut(&mut self) -> &mut nalgebra::Vector2<f32> { &mut self.position }
    pub fn text(&self) -> &Vec<TextObj> { &self.text }
    pub fn text_mut(&mut self) -> &mut Vec<TextObj> { &mut self.text }
    pub fn push_obj(&mut self, text: TextObj) { self.text.push(text) }
}

/// 描画するテキストの一時構造体
pub(super) struct TextEntrySection<'a> {
    pub bound: nalgebra::Vector2<f32>, 
    pub text: &'a mut TextEntry, 
}
impl<'a> From<TextEntrySection<'a>> for Section<'a> {
    fn from(value: TextEntrySection<'a>) -> Self { Section {
        screen_position: (value.text.position.x, value.text.position.y),
        bounds: (value.bound.x, value.bound.y),
        text: value.text.text.iter_mut()
            .map(|t| Text::from(t))
            .collect(),
        layout: value.text.layout
    }}
}