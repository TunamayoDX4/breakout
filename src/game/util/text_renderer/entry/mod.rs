//! テキストのエントリ

use wgpu_glyph::{
	Section, Text, Layout, BuiltInLineBreaker
};

/// テキストのオブジェクト
pub mod body;

/// テキストの領域サイズ
pub mod bound;

/// 描画するテキスト
pub struct TextObj {
    pub text: body::TextBody, 
    pub scale: f32, 
    pub color: [f32; 4], 
}
impl TextObj {
    pub fn new<B: Into<body::TextBody>> (
        text: B, 
        
        scale: f32, 
        color: [f32; 4], 
    ) -> Self { Self {
        text: text.into(), 
        scale, 
        color, 
    } }
}
impl<'a> From<&'a TextObj> for Text<'a> {
    fn from(value: &'a TextObj) -> Self {
        Text::new((&value.text).into())
            .with_scale(value.scale)
            .with_color(value.color)
    }
}

/// 描画するテキストのエントリー
pub struct TextEntry {
    bound_size: bound::TextBound, 
    position: nalgebra::Vector2<f32>, 
    text: Vec<TextObj>, 
    layout: Layout<BuiltInLineBreaker>, 
}
impl TextEntry {
    pub fn new<P>(
        bound_size: bound::TextBound, 
        position: P, 
        text: Vec<TextObj>, 
        layout: Layout<BuiltInLineBreaker>, 
    ) -> Self where
        P: Into<nalgebra::Vector2<f32>>
    { Self {
        bound_size, 
        position: position.into(),
        text,
        layout, 
    } }
    pub fn bound_size(&self) -> &bound::TextBound { &self.bound_size }
    pub fn bound_size_mut(&mut self) -> &mut bound::TextBound { &mut self.bound_size }
    pub fn position(&self) -> &nalgebra::Vector2<f32> { &self.position }
    pub fn position_mut(&mut self) -> &mut nalgebra::Vector2<f32> { &mut self.position }
    pub fn text(&self) -> &Vec<TextObj> { &self.text }
    pub fn text_mut(&mut self) -> &mut Vec<TextObj> { &mut self.text }
    pub fn push_obj(&mut self, text: TextObj) { self.text.push(text) }
}

/// 描画するテキストの一時構造体
pub(super) struct TextEntrySection<'a> {
    pub disp_size: nalgebra::Vector2<f32>, 
    pub text: &'a TextEntry, 
}
impl<'a> From<TextEntrySection<'a>> for Section<'a> {
    fn from(value: TextEntrySection<'a>) -> Self { 
        Section {
            screen_position: (value.text.position.x, value.text.position.y),
            bounds: value.text.bound_size.to_size(value.disp_size),
            text: value.text.text.iter()
                .map(|t| Text::from(t))
                .collect(),
            layout: value.text.layout
        }
    }
}