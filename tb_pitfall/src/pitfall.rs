use crate::sprites::load_harry_sprites;
use crate::types::*;

use serde_json;
use toybox_core;
use toybox_core::graphics::{load_digit_sprites, Color, FixedSpriteData};
use toybox_core::{graphics::Drawable, AleAction, QueryError};

const SIZE: (i32, i32) = (160, 210);
const OFFSET: (i32, i32) = (8, 6);

const DARK_GREEN: (u8, u8, u8) = (53, 95, 24);
const LIGHT_GREEN: (u8, u8, u8) = (110, 156, 66);

const TREES_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 51);
const BARK_COLOR: (u8, u8, u8) = (72, 72, 0);

const DIGIT_WIDTH: i32 = 8;

const SKY_XY: (i32, i32) = (OFFSET.0, 56);
const SKY_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 60);

const GROUND_XY: (i32, i32) = (OFFSET.0, 116);
const GROUND_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 16);
const GROUND_COLOR: (u8, u8, u8) = (187, 187, 53);

const ROOF_XY: (i32, i32) = (OFFSET.0, GROUND_XY.1 + GROUND_WH.1);
const ROOF_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 15);
const ROOF_COLOR: (u8, u8, u8) = UNDER_COLOR;

const UNDER_XY: (i32, i32) = (OFFSET.0, 180);
const UNDER_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 6);
const UNDER_COLOR: (u8, u8, u8) = (134, 134, 29);

const LOG_COLOR: (u8, u8, u8) = (105, 105, 15);

const POINTS_XY: (i32, i32) = (OFFSET.0 + 60, OFFSET.1 + 3);
const TIME_XY: (i32, i32) = (POINTS_XY.0, OFFSET.1 + 16);
const VBAR_XY: (i32, i32) = (OFFSET.0 + 16, OFFSET.1 + 16); // the || before the time

const SCREEN_X_BOUNDS: (i32, i32) = (8, 148);
const HARRY_WH: (i32, i32) = (0, 8);

const LADDER_PIT_WH: (i32, i32) = (8, 6);
const LADDER_PIT_OFF_Y: i32 = (GROUND_WH.1 - LADDER_PIT_WH.1) / 2;

const LADDER_BG_COLOR: (u8, u8, u8) = (0, 0, 0);
const LADDER_SQUARE_COLOR: (u8, u8, u8) = GROUND_COLOR;
const LADDER_SQUARE_WH: (i32, i32) = (4, 2);

const CENTER_LADDER_X: i32 = OFFSET.0 + 68;
const RIGHT_LOG_X: i32 = OFFSET.0 + 116;

// https://github.com/johnidm/asm-atari-2600/blob/8b613f3c4bc80d2dfec2c270395b0a97c992c9af/pitfall.asm#L3040-L3043
const JUMP_TABLE: &[i8] = &[
    1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, -1, 0, 0, 0, -1, 0, 0, -1, 0, -1, -1, -1, -1,
    -1, -1, -1,
];

lazy_static! {
    static ref DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/digits.txt"),
        Color::rgb(214, 214, 214),
        'X',
        '.'
    )
    .into_iter()
    .rev()
    .collect();
    static ref WALK_RIGHT_SPRITES: Vec<FixedSpriteData> =
        load_harry_sprites(include_str!("resources/harry-walk.txt"));
    static ref WALK_LEFT_SPRITES: Vec<FixedSpriteData> =
        WALK_RIGHT_SPRITES.iter().map(|it| it.flip_x()).collect();
    static ref STAND_RIGHT_SPRITE: FixedSpriteData =
        load_harry_sprites(include_str!("resources/harry-still.txt"))[0].clone();
    static ref STAND_LEFT_SPRITE: FixedSpriteData = STAND_RIGHT_SPRITE.flip_x();
    static ref JUMP_RIGHT_SPRITE: FixedSpriteData =
        load_harry_sprites(include_str!("resources/harry-jump.txt"))[0].clone();
    static ref JUMP_LEFT_SPRITE: FixedSpriteData = JUMP_RIGHT_SPRITE.flip_x();
    // two sprites for log animation
    static ref LOG_SPRITES: Vec<FixedSpriteData> =
        load_digit_sprites(include_str!("resources/log.txt"), Color::from(&LOG_COLOR), 'X', '.');
}

impl Default for Pitfall {
    fn default() -> Self {
        Pitfall {
            player_start: (GROUND_XY.0 + 12, 8 + GROUND_XY.1),
            frame_rate: 60,
        }
    }
}

impl Default for StateCore {
    fn default() -> Self {
        let cfg = Pitfall::default();
        Self::from_config(&cfg)
    }
}

impl StateCore {
    fn from_config(cfg: &Pitfall) -> Self {
        Self {
            score: 2000,
            frames_remaining: 60 * 20 * cfg.frame_rate,
            lives: 2,
            level: 1,
            player: Player {
                x: cfg.player_start.0,
                y: cfg.player_start.1,
                facing_left: false,
                action: PlayerAction::Stand,
            },
            entities: vec![
                Entity::Log { x: RIGHT_LOG_X },
                Entity::Ladder { x: CENTER_LADDER_X },
            ],
        }
    }
}

impl Player {
    fn walk(&mut self) {
        self.action = match self.action {
            PlayerAction::Jump(_) => return,
            PlayerAction::Fall | PlayerAction::Hurt | PlayerAction::Stand => PlayerAction::Walk(0),
            PlayerAction::Walk(x) => PlayerAction::Walk((x + 1) % 8),
        };
    }
    fn is_upstairs(&self) -> bool {
        self.y < ROOF_XY.1
    }
}

impl Entity {
    fn collides(&self, player: &Player) -> bool {
        match self {
            Entity::Log { x } => {
                let x2 = x + LOG_SPRITES[0].width();

                return player.is_upstairs() && player.x >= *x && player.x <= x2;
            }
            Entity::Ladder { x } => {
                let x2 = x + LADDER_PIT_WH.0;
                return player.x >= *x && player.x <= x2;
            }
        }
    }
}

impl toybox_core::Simulation for Pitfall {
    /// Seed simulation.
    fn reset_seed(&mut self, seed: u32) {
        // TODO
        // Implement this if your game has randomness (if it does not, you can skip)
        // This is typically done by adding a field onto the Pitfall struct
    }

    /// Generate a new State. This is in a Box<State> because it may be 1 of many unknown types as far as calling code is concerned.
    fn new_game(&mut self) -> Box<dyn toybox_core::State> {
        Box::new(State {
            config: self.clone(),
            state: StateCore::from_config(self),
        })
    }

    /// Generate a new State from JSON String (usually modified from a dump of State::to_json).
    fn new_state_from_json(
        &self,
        json: &str,
    ) -> Result<Box<dyn toybox_core::State>, serde_json::Error> {
        let state: StateCore = serde_json::from_str(json)?;
        Ok(Box::new(State {
            config: self.clone(),
            state,
        }))
    }

    /// Return a tuple of game size in pixels, e.g., (100,100).
    fn game_size(&self) -> (i32, i32) {
        SIZE
    }

    /// This serializes the "config" for a game to json.
    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Should be no JSON Serialization Errors.")
    }

    /// This deserializes the "config" for a game from json.
    /// Generate new state and new config from JSON String.
    fn from_json(&self, json: &str) -> Result<Box<dyn toybox_core::Simulation>, serde_json::Error> {
        let config: Pitfall = serde_json::from_str(json)?;
        Ok(Box::new(config))
    }

    /// Legal action set:
    fn legal_action_set(&self) -> Vec<AleAction> {
        // TODO
        // Update this; you may want to use ALE actions. See
        // other games for examples.
        vec![]
    }

    /// Getter for JSON Schema for this game's state.
    fn schema_for_state(&self) -> String {
        let schema = schema_for!(StateCore);
        serde_json::to_string(&schema).expect("JSONSchema should be flawless.")
    }

    /// Getter for JSON Schema for this game's config.
    fn schema_for_config(&self) -> String {
        let schema = schema_for!(Pitfall);
        serde_json::to_string(&schema).expect("JSONSchema should be flawless.")
    }
}

impl toybox_core::State for State {
    /// When < 0, this state should be replaced with a call to new_game() on the simulation.
    fn lives(&self) -> i32 {
        self.state.lives
    }
    /// Get the score from the game, i32 allows for negative scores.
    fn score(&self) -> i32 {
        self.state.score
    }
    /// Get the level from the game.
    fn level(&self) -> i32 {
        self.state.level
    }
    /// To update internally to the next state, we pass buttons to internal logic.
    fn update_mut(&mut self, buttons: toybox_core::Input) {
        // subtract from time-limit.
        self.state.frames_remaining -= 1;

        let update = if let PlayerAction::Jump(index) = self.state.player.action {
            self.state.player.y -= JUMP_TABLE[index] as i32;
            if buttons.left {
                self.state.player.x -= 1;
                self.state.player.facing_left = true;
            } else if buttons.right {
                self.state.player.x += 1;
                self.state.player.facing_left = false;
            }
            if index + 1 >= JUMP_TABLE.len() {
                Some(PlayerAction::Stand)
            } else {
                Some(PlayerAction::Jump(index + 1))
            }
        } else {
            None
        };
        if let Some(new_action) = update {
            self.state.player.action = new_action;
        }

        // Can only jump if standing or walking:
        match self.state.player.action {
            PlayerAction::Stand | PlayerAction::Walk(_) => {
                if buttons.button1 {
                    self.state.player.action = PlayerAction::Jump(0);
                }
            }
            PlayerAction::Jump(_) | PlayerAction::Hurt | PlayerAction::Fall => {}
        }

        match self.state.player.action {
            PlayerAction::Stand | PlayerAction::Hurt | PlayerAction::Walk(_) => {
                if buttons.left {
                    self.state.player.x -= 1;
                    self.state.player.facing_left = true;
                    self.state.player.walk();
                } else if buttons.right {
                    self.state.player.x += 1;
                    self.state.player.facing_left = false;
                    self.state.player.walk();
                } else {
                    self.state.player.action = PlayerAction::Stand;
                }
                for e in self.state.entities.iter() {
                    if e.collides(&self.state.player) {
                        match e {
                            Entity::Log { .. } => {
                                self.state.player.action = PlayerAction::Hurt;
                                self.state.score -= 1;
                            }
                            Entity::Ladder { .. } => {
                                if self.state.player.is_upstairs() {
                                    self.state.player.action = PlayerAction::Fall;
                                    self.state.score -= 100;
                                } else {
                                    if buttons.up {
                                        // TODO: climbing
                                    }
                                }
                            }
                        }
                    }
                }
            }
            PlayerAction::Jump(_) | PlayerAction::Fall => {}
        }

        // falling:
        if let PlayerAction::Fall = self.state.player.action {
            self.state.player.y += 1;
            if self.state.player.y >= UNDER_XY.1 {
                self.state.player.y = UNDER_XY.1;
                self.state.player.action = PlayerAction::Stand;
            }
        }

        // don't let scores go too low...
        if self.state.score < 0 {
            self.state.score = 0;
        }
    }
    /// Any state can create a vector of drawable objects to present itself.
    fn draw(&self) -> Vec<Drawable> {
        let mut out = Vec::new();

        out.push(Drawable::Clear(Color::black()));

        // This is the dark foliage (score/time rendered here)
        out.push(Drawable::Rectangle {
            color: Color::from(&DARK_GREEN),
            x: OFFSET.0,
            y: OFFSET.1,
            w: TREES_WH.0,
            h: TREES_WH.1,
        });

        // This is the bright foliage (tree barks go on top)
        out.push(Drawable::Rectangle {
            color: Color::from(&LIGHT_GREEN),
            x: SKY_XY.0,
            y: SKY_XY.1,
            w: SKY_WH.0,
            h: SKY_WH.1,
        });

        // This is the floor in the 'underground'.
        out.push(Drawable::Rectangle {
            color: Color::from(&UNDER_COLOR),
            x: UNDER_XY.0,
            y: UNDER_XY.1,
            w: UNDER_WH.0,
            h: UNDER_WH.1,
        });

        // This is the roof in the 'underground'.
        out.push(Drawable::Rectangle {
            color: Color::from(&ROOF_COLOR),
            x: ROOF_XY.0,
            y: ROOF_XY.1,
            w: ROOF_WH.0,
            h: ROOF_WH.1,
        });

        // this is the floor where 'Harry' starts the game.
        out.push(Drawable::Rectangle {
            color: Color::from(&GROUND_COLOR),
            x: GROUND_XY.0,
            y: GROUND_XY.1,
            w: GROUND_WH.0,
            h: GROUND_WH.1,
        });

        for e in self.state.entities.iter() {
            match e {
                Entity::Log { x } => {
                    out.push(Drawable::StaticSprite {
                        x: *x,
                        y: GROUND_XY.1 + 1,
                        data: LOG_SPRITES[0].clone(),
                    });
                }
                Entity::Ladder { x } => {
                    let ladder_x = *x;

                    // black bg:
                    let start_y = GROUND_XY.1 + LADDER_PIT_OFF_Y;
                    out.push(Drawable::Rectangle {
                        color: Color::from(&LADDER_BG_COLOR),
                        x: ladder_x,
                        y: start_y,
                        w: LADDER_PIT_WH.0,
                        h: UNDER_XY.1 - start_y,
                    });
                    let rungs_start = GROUND_XY.1 + GROUND_WH.1 + 1;

                    for y in (rungs_start..UNDER_XY.1).step_by(6) {
                        out.push(Drawable::Rectangle {
                            color: Color::from(&LADDER_SQUARE_COLOR),
                            x: ladder_x + 2,
                            y: y + 2,
                            w: LADDER_SQUARE_WH.0,
                            h: LADDER_SQUARE_WH.1,
                        });
                    }
                }
            }
        }

        // draw the player
        let harry_left = self.state.player.facing_left;
        let mut harry_img: &FixedSpriteData = if harry_left {
            &STAND_LEFT_SPRITE
        } else {
            &STAND_RIGHT_SPRITE
        };

        // render player:
        match self.state.player.action {
            PlayerAction::Stand => {}
            PlayerAction::Walk(frame) => {
                let frame = frame / 2;
                if harry_left {
                    harry_img = &WALK_LEFT_SPRITES[frame];
                } else {
                    harry_img = &WALK_RIGHT_SPRITES[frame];
                }
            }
            PlayerAction::Jump(_) | PlayerAction::Hurt | PlayerAction::Fall => {
                if harry_left {
                    harry_img = &JUMP_LEFT_SPRITE;
                } else {
                    harry_img = &JUMP_RIGHT_SPRITE;
                }
            }
        }

        out.push(Drawable::StaticSprite {
            x: self.state.player.x - harry_img.width() / 2,
            y: self.state.player.y - harry_img.height(),
            data: harry_img.clone(),
        });

        // draw pit-fronts:
        for e in self.state.entities.iter() {
            if let Entity::Ladder { x } = e {
                let start_y = GROUND_XY.1 + LADDER_PIT_OFF_Y;
                out.push(Drawable::Rectangle {
                    color: Color::from(&GROUND_COLOR),
                    x: *x,
                    y: start_y + LADDER_PIT_WH.1,
                    w: LADDER_PIT_WH.0,
                    h: LADDER_PIT_OFF_Y,
                });
            }
        }

        // helper for rendering numeric digits:
        fn render_digits(text: &str, x: i32, y: i32, out: &mut Vec<Drawable>) {
            for (i, digit) in text
                .chars()
                .rev()
                .map(|it| it.to_digit(10).unwrap_or_default())
                .enumerate()
            {
                let i = i as i32;
                let data: &FixedSpriteData = &DIGIT_SPRITES[digit as usize];
                out.push(Drawable::StaticSprite {
                    x: x - (i * DIGIT_WIDTH),
                    y,
                    data: data.clone(),
                })
            }
        }
        render_digits(
            &format!("{}", self.state.score),
            POINTS_XY.0,
            POINTS_XY.1,
            &mut out,
        );

        let frames_left = self.state.frames_remaining;
        let seconds_left = frames_left / self.config.frame_rate;
        let minutes_left = seconds_left / 60;
        let minutes_str = format!("{:02}", minutes_left.max(0));
        let seconds_str = format!("{:02}", seconds_left.max(0) % 60);

        render_digits(&seconds_str, TIME_XY.0, TIME_XY.1, &mut out);
        out.push(Drawable::StaticSprite {
            x: TIME_XY.0 - DIGIT_WIDTH * 2,
            y: TIME_XY.1,
            data: DIGIT_SPRITES[10].clone(),
        });
        render_digits(
            &minutes_str,
            TIME_XY.0 - DIGIT_WIDTH * 3,
            TIME_XY.1,
            &mut out,
        );

        out
    }
    /// Any state can serialize to JSON String.
    fn to_json(&self) -> String {
        serde_json::to_string(&self.state).expect("Should be no JSON Serialization Errors.")
    }
    /// Copy this state to save it for later.
    fn copy(&self) -> Box<dyn toybox_core::State> {
        Box::new(self.clone())
    }
    /// Submit a query to this state object, returning a JSON String or error message.
    fn query_json(&self, query: &str, args: &serde_json::Value) -> Result<String, QueryError> {
        // TODO
        // This is used for fast object inspection and is not necessary. Only implement
        // once you have finished game development and need higher-performance observations
        // of game features/objects.
        Ok("TODO".to_string())
    }
}
