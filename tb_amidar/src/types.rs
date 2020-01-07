use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toybox_core::graphics::Color;
use toybox_core::random;
use toybox_core::Direction;

use std::collections::{HashSet, VecDeque};

/// This struct represents the configuration of an Amidar game, and affects any new games generated from it.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Amidar {
    /// The random number generator that seeds new games.
    pub rand: random::Gen,
    /// A representation of the board as a list of strings.
    pub board: Vec<String>,
    /// Where does the player start on a new life?
    pub player_start: TilePoint,
    /// What is the background color?
    pub bg_color: Color,
    /// What is the player rectangle color?
    pub player_color: Color,
    /// What color are unpainted tiles?
    pub unpainted_color: Color,
    /// What color are painted tiles?
    pub painted_color: Color,
    /// What color are enemies?
    pub enemy_color: Color,
    /// What color do we fill the rectangles with when the player paints their circumference.
    pub inner_painted_color: Color,
    /// How many lives do new games start with?
    pub start_lives: i32,
    /// How many jumps do new games start with?
    pub start_jumps: i32,
    /// Should we show images/sprites (true) or just colored rectangles (false).
    pub render_images: bool,
    /// How long does chase mode last for, in frames?
    pub chase_time: i32,
    /// How much is eating an enemy in chase/chickens mode worth?
    pub chase_score_bonus: i32,
    /// How long does the invulnerable jump_time last?
    pub jump_time: i32,
    /// How many points do you get for filling a box?
    pub box_bonus: i32,
    /// This should be false if you ever use a non-default board.
    pub default_board_bugs: bool,
    /// What AIs should we use to spawn enemies on a new game?
    pub enemies: Vec<MovementAI>,
    /// How many previous junctions should the player and enemies remember?
    pub history_limit: u32,
    /// How fast do enemies move?
    pub enemy_starting_speed: i32,
    /// How fast does the player move?
    pub player_speed: i32,
}

/// When things are drawn, they are drawn in screen coordinates, i.e., pixels.
#[derive(Debug, Clone)]
pub struct ScreenPoint {
    pub sx: i32,
    pub sy: i32,
}

/// Strongly-typed vector for "world" positioning in Amidar. World points are larger than screen points because players/enemies often move fractions of a pixel per frame.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WorldPoint {
    pub x: i32,
    pub y: i32,
}

/// Strongly-typed vector for "tile" positioning in Amidar. These coordinates are related to world and screen points, but are more useful for addressing specific painted/unpainted tiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TilePoint {
    pub tx: i32,
    pub ty: i32,
}

/// This represents the boxes on the board, whether they are part of chickens/chase mode and whether they are filled in or not.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GridBox {
    /// Dimension of the GridBox: which tile is its top-left? Specifies location.
    pub top_left: TilePoint,
    /// Dimension of the GridBox: which tile is its bottom-right? Specifies size implicitly.
    pub bottom_right: TilePoint,
    /// Is this GridBox painted? This is computable from all the board points around the edge of the rectangle, but its much faster/cheaper to cache it here.
    pub painted: bool,
    /// Is this one of the four corner GridBoxes in the default map? If so, when you fill all of them in, you trigger chase mode!
    pub triggers_chase: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Tile {
    /// Tiles in the middle of the path that are not walkable are called empty; treated like walls for collision.
    Empty,
    /// Walkable tiles you haven't painted yet.
    Unpainted,
    /// Walkable tiles you have painted.
    ChaseMarker,
    /// During play, the same as Empty; Walkable tiles you haven't painted yet. Used to form GridBox objects with triggers_chase=true.
    Painted,
}

/// MovementAI represents Mob (enemy/player) logic for movement.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, JsonSchema)]
pub enum MovementAI {
    /// Movement is based upon input commands.
    Player,
    /// Movement is based upon tabular representation of Atari movement. Does not adapt to different boards.
    EnemyLookupAI {
        /// How far through the table row are we?
        next: u32,
        /// Which table row should we use?
        default_route_index: u32,
    },
    /// Movement is procedural: looping around the perimiter.
    EnemyPerimeterAI {
        /// Where to start on the perimeter.
        start: TilePoint,
    },
    /// This movement is based on the Wikipedia description; alternating up and down based on when it collides.
    EnemyAmidarMvmt {
        /// Current vertical desire: up or down.
        vert: Direction,
        /// Current horizontal desire: left or right.
        horiz: Direction,
        /// Do we start up or down?
        start_vert: Direction,
        /// Do we start left or right?
        start_horiz: Direction,
        /// Where do I start?
        start: TilePoint,
    },
    /// At every junction, an enemy chooses a random legal direction and proceeds in that direction until hitting the next junction.
    EnemyRandomMvmt {
        /// Where do I start?
        start: TilePoint,
        /// Which direction to move first?
        start_dir: Direction,
        /// Which direction am I currently moving?
        dir: Direction,
    },
    /// Move randomly unless the player is within some fixed Manhattan distance of this enemy -- in that case, move toward the player.
    EnemyTargetPlayer {
        /// Where do I start?
        start: TilePoint,
        /// Which direction do I explore first?
        start_dir: Direction,
        /// How far (Manhattan distance) can I see?
        vision_distance: i32,
        /// Which direction am I currently moving?
        dir: Direction,
        /// We lock onto a player's position when we see it, so that we can actually be evaded.
        player_seen: Option<TilePoint>,
    },
}

/// Mob is a videogame slang for "mobile" unit. Players and Enemies are the same struct.
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Mob {
    /// How is this unit controlled?
    pub ai: MovementAI,
    /// Where is this unit placed (WorldPoint represents sub-pixels!)
    pub position: WorldPoint,
    /// Have I been caught/eaten in chase mode?
    pub caught: bool,
    /// How fast do I get to move?
    pub speed: i32,
    /// Am I currently moving toward a point?
    pub step: Option<TilePoint>,
    /// Which junctions have I visited most recently?
    pub history: VecDeque<u32>,
}

/// Board represents the Amidar level/board and all associated information.
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Board {
    /// What are the state of the tiles on the board: rows first, then columns.
    pub tiles: Vec<Vec<Tile>>,
    /// How wide is the board?
    pub width: u32,
    /// How tall is the board?
    pub height: u32,
    /// Which positions (y*width + x) are junctions? Helps MovementAI and painting game logic!
    pub junctions: HashSet<u32>,
    /// Which junctions trigger chases?
    pub chase_junctions: HashSet<u32>,
    /// The list of boxes (inside-portions) of the board.
    pub boxes: Vec<GridBox>,
}

/// This struct is temporarily used inside of the game logic, to ensure purely-functional behavior in certain points. Encodes any changes to the board that could happen in a single update.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, JsonSchema)]
pub struct BoardUpdate {
    /// Number of vertical segments filled in.
    pub vertical: i32,
    /// Number of horizontal segments filled in.
    pub horizontal: i32,
    /// The number of boxes filled in.
    pub num_boxes: i32,
    /// Whether we just triggered chase mode or not.
    pub triggers_chase: bool,
    /// If we just painted something, the start junction and the end junction as a tuple!
    pub junctions: Option<(u32, u32)>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct StateCore {
    /// Where are random numbers drawn from?
    pub rand: random::Gen,
    /// How many points have the player earned?
    pub score: i32,
    /// How many lives does the player posess?
    pub lives: i32,
    /// What is the current level? 1-based.
    pub level: i32,
    /// How many jumps are still available to the player?
    pub jumps: i32,
    /// When non-zero, the player has triggered 'chase mode' and we are counting down to when it expires.
    pub chase_timer: i32,
    /// When non-zero, the player has executed a jump and we are counting down to when it expires.
    pub jump_timer: i32,
    /// The position and state of the player.
    pub player: Mob,
    /// The position and other state for the enemies.
    pub enemies: Vec<Mob>,
    /// A representation of the current game board.
    pub board: Board,
}

/// Wrapping the current game config into one struct with the current frame state.
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct State {
    /// The config that generated the original state for this game.
    pub config: Amidar,
    /// The state that represents the immediately current frame.
    pub state: StateCore,
}

/// When we compared the player position to all the enemies, what happened?
#[derive(PartialEq, Eq, Clone, Copy, JsonSchema)]
pub enum EnemyPlayerState {
    /// Most of the time: nobody's colliding.
    Miss,
    /// The player just died.
    PlayerDeath,
    /// In chase mode, the player just caught the given enemy (id by index in state.enemies list!)
    EnemyCatch(usize),
}
