//! ゲームの状態

use std::io::Read;

use super::util::text_renderer::TextRendererGMArc;

pub struct GameState {
    pub font: TextRendererGMArc, 
}
impl GameState {
    pub(super) fn new(
        gfx_ctx: &crate::gfx::WGContext, 
    ) -> anyhow::Result<Self> { 
        let mut ttf_bytes = Vec::new();
        let mut fp = std::fs::File::open("font/Mplus1Code-Medium.ttf")?;
        ttf_bytes.resize(fp.metadata()?.len() as usize, 0);
        fp.read(&mut ttf_bytes)?;
        let font = super::util::text_renderer::TextRendererGMArc::new(
            gfx_ctx, 
            ttf_bytes
        )?;
        Ok(Self {
            font, 
        })
    }
}