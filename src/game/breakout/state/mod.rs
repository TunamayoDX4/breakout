//! ゲームの状態

use std::sync::Arc;

use parking_lot::Mutex;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Yes, 
    GameOver, 
    GameClear, 
}

/// 難易度
pub enum BreakOutDifficulity {
    Easy, 
    Normal, 
    Hard, 
}

pub struct BreakOutGameState {
    /// 残弾数
    pub(super) remain_ball: u32, 
    /// 状態
    pub(super) state: GameState, 
    /// スコア
    pub score: std::sync::Arc<parking_lot::Mutex<u64>>, 
    /// 難易度
    pub difficulity: BreakOutDifficulity, 
}
impl BreakOutGameState {
    pub fn new() -> Self { Self {
        remain_ball: 5,
        state: GameState::Yes,
        score: Arc::new(Mutex::new(0)), 
        difficulity: BreakOutDifficulity::Easy, 
    }}
}