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

const SKY_XY: (i32, i32) = (OFFSET.0, 56);
const SKY_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 60);

const GROUND_XY: (i32, i32) = (OFFSET.0, 116);
const GROUND_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 16);
const GROUND_COLOR: (u8, u8, u8) = (187, 187, 53);

const UNDER_XY: (i32, i32) = (OFFSET.0, 180);
const UNDER_WH: (i32, i32) = (SIZE.0 - OFFSET.0, 6);
const UNDER_COLOR: (u8, u8, u8) = (134, 134, 29);

const LOG_COLOR: (u8, u8, u8) = (105, 105, 15);

const POINTS_XY: (i32, i32) = (OFFSET.0 + 60, OFFSET.1 + 3);
const TIME_XY: (i32, i32) = (POINTS_XY.0, OFFSET.1 + 16);
const VBAR_XY: (i32, i32) = (OFFSET.0 + 16, OFFSET.1 + 16); // the || before the time

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
}

impl Default for Pitfall {
    fn default() -> Self {
        Pitfall {
            // TODO
            // In types.rs, there will be a struct called Pitfall
            // You should populate it with the necessary game-specific fields 
            // and their types, and document them there. 
            // Then add default values for those fields here.
        }
    }
}

impl Default for StateCore {
    fn default() -> Self {
        StateCore {
            score: 0,
            lives: 1000,
            level: 1,
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
            state: StateCore::default(),
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
        if buttons.button1 {
            // TODO: jump
        }
    }
    /// Any state can create a vector of drawable objects to present itself.
    fn draw(&self) -> Vec<Drawable> {
        let mut out = Vec::new();

        out.push(Drawable::Clear(Color::black()));

        out.push(Drawable::Rectangle {
            color: Color::from(&DARK_GREEN),
            x: OFFSET.0,
            y: OFFSET.1,
            w: TREES_WH.0,
            h: TREES_WH.1,
        });

        out.push(Drawable::Rectangle {
            color: Color::from(&LIGHT_GREEN),
            x: SKY_XY.0,
            y: SKY_XY.1,
            w: SKY_WH.0,
            h: SKY_WH.1,
        });

        out.push(Drawable::Rectangle {
            color: Color::from(&UNDER_COLOR),
            x: UNDER_XY.0,
            y: UNDER_XY.1,
            w: UNDER_WH.0,
            h: UNDER_WH.1,
        });

        out.push(Drawable::Rectangle {
            color: Color::from(&GROUND_COLOR),
            x: GROUND_XY.0,
            y: GROUND_XY.1,
            w: GROUND_WH.0,
            h: GROUND_WH.1,
        });

        let numbers: &[FixedSpriteData] = &DIGIT_SPRITES;
        for (i, digit) in [0usize, 0, 0, 2].iter().enumerate() {
            let i = i as i32;
            let data: &FixedSpriteData = &numbers[*digit];
            out.push(Drawable::StaticSprite {
                x: POINTS_XY.0 - (i * 8),
                y: POINTS_XY.1,
                data: data.clone(),
            })
        }

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
