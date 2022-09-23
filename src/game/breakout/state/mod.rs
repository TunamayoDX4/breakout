//! ゲームの状態

use std::sync::Arc;

use parking_lot::Mutex;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Yes, 
    GameOver, 
    GameClear, 
}

pub struct BreakOutGameState {
    /// 残弾数
    pub(super) remain_ball: u32, 
    /// 状態
    pub(super) state: GameState, 
    /// スコア
    pub(super) score: std::sync::Arc<parking_lot::Mutex<u64>>, 
}
impl BreakOutGameState {
    pub fn new() -> Self { Self {
        remain_ball: 5,
        state: GameState::Yes,
        score: Arc::new(Mutex::new(0)), 
    }}
}