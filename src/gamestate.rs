use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, StageLabel)]
pub enum GameStage {
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Main,
    Cauldron,
}
