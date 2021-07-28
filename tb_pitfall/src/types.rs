use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Pitfall {
    /// Where does the player start (x,y)?
    pub player_start: (i32, i32),
    /// How many frames per second (timer presented in seconds).
    pub frame_rate: i32,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Player {
    /// X position of the player.
    pub x: i32,
    /// Y position of the player.
    pub y: i32,
    /// Direction to face.
    pub facing_left: bool,
    /// Walk/Jump/Stand?
    pub action: PlayerAction,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub enum PlayerAction {
    Stand,
    Walk(usize),
    Jump(usize),
    Hurt,
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

    /// What is the time remaining (in frames?)
    pub frames_remaining: i32,

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

pub struct Ladder {
    x: i32,
}
