//! テキスト専用のレンダラ

//! テキストのレンダラ

use std::sync::Arc;

use hashbrown::HashMap;
use wgpu_glyph::{Section, Text, Layout, BuiltInLineBreaker};
type PMutex<T> = parking_lot::Mutex<T>;

/// 描画するテキスト
pub struct TextObj {
    pub text: std::borrow::Cow<'static, str>, 
    pub scale: f32, 
    pub color: [f32; 4], 
}
impl<'a> From<&'a TextObj> for Text<'a> {
    fn from(value: &'a TextObj) -> Self {
        Text::new(&value.text)
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
struct TextEntrySection<'a> {
    pub bound: nalgebra::Vector2<f32>, 
    pub text: &'a TextEntry, 
}
impl<'a> From<TextEntrySection<'a>> for Section<'a> {
    fn from(value: TextEntrySection<'a>) -> Self { Section {
        screen_position: (value.text.position.x, value.text.position.y),
        bounds: (value.bound.x, value.bound.y),
        text: value.text.text.iter()
            .map(|t| Text::from(t))
            .collect(),
        layout: value.text.layout
    }}
}

/// テキストのレンダラのフォントモジュール
struct TextRendererGlyphModule {
    glyph_brush: wgpu_glyph::GlyphBrush<()>, 
}
impl TextRendererGlyphModule {
    pub fn new(
        gfx_ctx: &crate::gfx::WGContext, 
        ttf_bytes: Vec<u8>, 
    ) -> anyhow::Result<TextRendererGMArc> {

        let font = wgpu_glyph::ab_glyph::FontArc::try_from_vec(
            ttf_bytes
        )?;

        let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font)
            .build(&gfx_ctx.device, gfx_ctx.config.format);

        Ok(TextRendererGMArc(Arc::new(PMutex::new(Self {
            glyph_brush
        }))))
    }
}

/// テキストのレンダラのフォントモジュールの共有構造体
#[derive(Clone)]
pub struct TextRendererGMArc (Arc<PMutex<TextRendererGlyphModule>>);
impl TextRendererGMArc {
    pub fn new(
        gfx_ctx: &crate::gfx::WGContext, 
        ttf_bytes: Vec<u8>, 
    ) -> anyhow::Result<TextRendererGMArc> {
        TextRendererGlyphModule::new(gfx_ctx, ttf_bytes)
    }
}

/// テキストのレンダラ
pub struct TextRenderer {
    staging_belt: wgpu::util::StagingBelt, 
    entries: HashMap<std::borrow::Cow<'static, str>, TextEntry>, 
    glyph: TextRendererGMArc 
}
impl TextRenderer {
    pub fn new(
        gfx_ctx: &crate::gfx::WGContext, 
        entries: Option<HashMap<std::borrow::Cow<'static, str>, TextEntry>>, 
        glyph:  TextRendererGMArc , 
    ) -> anyhow::Result<Self> { 

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let entries = entries.map_or_else(
            || HashMap::new(), 
            |o| o
        );

        Ok(Self {
            staging_belt,
            entries, 
            glyph, 
        })
    }
    pub fn set_glyph(&mut self, glyph: TextRendererGMArc) {
        self.glyph = glyph
    }
}
impl crate::gfx::WGRenderer for TextRenderer {
    fn rendering(
        &mut self, 
        output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        ctx: &crate::gfx::WGContext, 
    ) {
        let mut enc = ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Text renderer")
            }
        );

        {
            let _ = enc.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Text render pass"), 
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view,
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
        }

        let bound = nalgebra::Vector2::new(
            ctx.size.width as f32, ctx.size.height as f32
        );
        self.entries.iter()
            .map(|(_, text)| TextEntrySection {
                bound,
                text: &text,
            })
            .map(|section| Section::from(section))
            .for_each(|section| {
                self.glyph.0.lock().glyph_brush.queue(section);
            });

        self.glyph.0.lock().glyph_brush.draw_queued(
            &ctx.device, 
            &mut self.staging_belt, 
            &mut enc, 
            view, 
            ctx.size.width, 
            ctx.size.height
        ).expect("Text draw queued");

        self.staging_belt.finish();
        ctx.queue.submit([enc.finish()]);
        self.staging_belt.recall();
    }
}