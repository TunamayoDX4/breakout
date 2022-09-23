//! ゲームの状態

use std::io::Read;

use super::util::text_renderer::TextRendererGMArc;

pub struct GameState {
    pub ipaexg: TextRendererGMArc, 
}
impl GameState {
    pub(super) fn new(
        gfx_ctx: &crate::gfx::WGContext, 
    ) -> anyhow::Result<Self> { 
        let mut ttf_bytes = Vec::new();
        let mut fp = std::fs::File::open("font/ipaexg.ttf")?;
        ttf_bytes.resize(fp.metadata()?.len() as usize, 0);
        fp.read(&mut ttf_bytes)?;
        let ipaexg = super::util::text_renderer::TextRendererGMArc::new(
            gfx_ctx, 
            ttf_bytes
        )?;
        Ok(Self {
            ipaexg, 
        })
    }
}