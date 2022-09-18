//! ゲームの状態

use super::util::text_renderer::TextRendererGMArc;

pub struct GameState {
    pub ipaexg: TextRendererGMArc, 
}
impl GameState {
    pub(super) fn new(
        gfx_ctx: &crate::gfx::WGContext, 
    ) -> anyhow::Result<Self> { 
        let ipaexg = super::util::text_renderer::TextRendererGMArc::new(
            gfx_ctx, 
            include_bytes!("../../../ipaexg.ttf").to_vec(), 
        )?;
        Ok(Self {
            ipaexg, 
        })
    }
}