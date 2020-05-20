use crate::Body2D;
use toybox_core::graphics::Color;

/// This represents the setup needed for a game of Pong.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PongConfig {
    /// What is the background color of the board? Brownish by default.
    pub bg_color: Color,
    /// The gray area at the top/bottom we refer to as the "frame".
    pub frame_color: Color,
    /// The ball is the same color as the frame.
    pub ball_color: Color,
    /// Player 1 (the player) Color (greenish).
    pub p1_color: Color,
    /// Player 2 (the AI) color (orangish/pink).
    pub p2_color: Color,
    /// Number of points in a game.
    pub game_points: i32,
    /// Paddle speed.
    pub paddle_speed: f64,
    /// Maximum paddle_speed:
    pub max_paddle_speed: f64,
}

/// This represents the per-frame snapshot of mutable state in a Pong game.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FrameState {
    /// Are we about to reset the ball?
    pub reset: bool,
    /// How many points has the player earned?
    pub p1_score: i32,
    /// How many points has the opponent/AI earned?
    pub p2_score: i32,
    /// Ball position describes the center of the ball.
    pub ball: Body2D,
    /// Where is the player's paddle?
    pub p1_paddle: Body2D,
    /// Where is the enemy's paddle?
    pub p2_paddle: Body2D,
}

/// The pong game's true state has it's initial configuration that launched the game, and it's current state.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct State {
    /// Setup that created this game; effectively immutable here.
    pub config: PongConfig,
    /// All the data known about this frame.
    pub state: FrameState,
}
