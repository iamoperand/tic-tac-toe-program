use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive,
)]
pub enum Sign {
    X,
    O,
}

#[account]
pub struct Game {
    pub players: [Pubkey; 2],          // 32
    pub turn: u8,                      // 1
    pub board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1)
    pub state: GameState,              // 32 + 1
}
