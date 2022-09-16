//! ゲームの状態

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
}
impl BreakOutGameState {
    pub fn new() -> Self { Self {
        remain_ball: 5,
        state: GameState::Yes,
    }}
}