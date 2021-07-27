use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Pitfall {
    // Fill in the high-level fields of your game here
    pub player_start: (i32, i32),
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Player {
    pub x: i32,
    pub y: i32,
}

// Put the rest of the structs for your game here

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct StateCore {
    /// How many points have the player earned?
    pub score: i32,
    /// How many lives does the player posess?
    pub lives: i32,
    /// What is the current level? 1-based.
    pub level: i32,

    /// Where is the player, is he jumping, etc.
    pub player: Player,
}

/// Wrapping the current game config into one struct with the current frame state.
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct State {
    /// The config that generated the original state for this game.
    pub config: Pitfall,
    /// The state that represents the immediately current frame.
    pub state: StateCore,
}
