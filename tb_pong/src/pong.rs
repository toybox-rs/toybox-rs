use crate::types::*;
use crate::{Body2D, Vec2D};
use toybox_core::collision::Rect;
use toybox_core::{
    graphics::{Color, Drawable},
    AleAction,
};

mod screen {
    pub const GAME_SIZE: (i32, i32) = (160, 210);
    pub const HEADER_H: i32 = 24;
    pub const HEADER_Y: i32 = 0;
    pub const TOP_FRAME_H: i32 = 10;
    pub const TOP_FRAME_Y: i32 = HEADER_H + HEADER_Y;
    pub const BOTTOM_FRAME_H: i32 = 16;
    pub const BOTTOM_FRAME_Y: i32 = GAME_SIZE.1 - BOTTOM_FRAME_H;
    pub const PADDLE_SHAPE: (i32, i32) = (4, 16);
    pub const P1_START_POSITION: (i32, i32) = (140, 96);
    // shows up frame0058.png
    pub const P2_START_POSITION: (i32, i32) = (16, 116);
    pub const BALL_START_POSITION: (i32, i32) = (78, 116);
    pub const BALL_SHAPE: (i32, i32) = (2, 4);
    pub const BALL_START_VELOCITY: (i32, i32) = (-2, 1);
}

impl Default for PongConfig {
    fn default() -> Self {
        PongConfig {
            p1_color: Color::rgb(92, 186, 92),
            p2_color: Color::rgb(231, 130, 74),
            bg_color: Color::rgb(144, 72, 17),
            ball_color: Color::rgb(236, 236, 236),
            frame_color: Color::rgb(236, 236, 236),
            // 4 feels right compared to human_play_ale.
            paddle_speed: 4.0,
            game_points: 21,
        }
    }
}

impl toybox_core::Simulation for PongConfig {
    fn reset_seed(&mut self, _seed: u32) {
        // No randomness in Pong.
    }
    fn new_game(&mut self) -> Box<dyn toybox_core::State> {
        let (ball_sx, ball_sy) = screen::BALL_START_POSITION;
        let (ball_dx, ball_dy) = screen::BALL_START_VELOCITY;
        let state = FrameState {
            reset: true,
            p1_score: 0,
            p2_score: 0,
            ball: Body2D::new_detailed(
                ball_sx as f64,
                ball_sy as f64,
                ball_dx as f64,
                ball_dy as f64,
            ),
            p1_paddle: Body2D::new_pos(
                screen::P1_START_POSITION.0 as f64,
                screen::P1_START_POSITION.1 as f64,
            ),
            p2_paddle: Body2D::new_pos(
                screen::P2_START_POSITION.0 as f64,
                // start off-screen
                -100.0,
            ),
        };
        Box::new(State {
            config: self.clone(),
            state,
        })
    }
    fn new_state_from_json(
        &self,
        json: &str,
    ) -> Result<Box<dyn toybox_core::State>, serde_json::Error> {
        Ok(Box::new(serde_json::from_str::<State>(json)?))
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn from_json(&self, json: &str) -> Result<Box<dyn toybox_core::Simulation>, serde_json::Error> {
        Ok(Box::new(serde_json::from_str::<PongConfig>(json)?))
    }
    /// Sync with [ALE Impl](https://github.com/mgbellemare/Arcade-Learning-Environment/blob/master/src/games/supported/Pong.cpp#L47)
    /// Note, leaving a call to sort in this impl to remind users that these vecs are ordered!
    fn legal_action_set(&self) -> Vec<AleAction> {
        let mut actions = vec![
            AleAction::NOOP,
            // IDK why fire is in here.
            AleAction::FIRE,
            AleAction::LEFT,
            AleAction::RIGHT,
            AleAction::LEFTFIRE,
            AleAction::RIGHTFIRE,
        ];
        actions.sort();
        actions
    }
    fn schema_for_state(&self) -> String {
        let schema = schema_for!(State);
        serde_json::to_string(&schema).expect("JSONSchema should be flawless.")
    }
    fn schema_for_config(&self) -> String {
        let schema = schema_for!(PongConfig);
        serde_json::to_string(&schema).expect("JSONSchema should be flawless.")
    }
}

impl toybox_core::State for State {
    fn lives(&self) -> i32 {
        // how many more points can p1 lose?
        self.config.game_points - self.state.p2_score
    }
    fn score(&self) -> i32 {
        // how many points do we have?
        self.state.p1_score
    }
    fn level(&self) -> i32 {
        0
    }
    fn update_mut(&mut self, buttons: toybox_core::Input) {
        if self.state.reset {
            // reset enemy paddle.
            self.state.p2_paddle.position.y = screen::P2_START_POSITION.1 as f64;
            // re-launch ball:
            self.state.ball.position.x = screen::BALL_START_POSITION.0 as f64;
            self.state.ball.position.y = screen::BALL_START_POSITION.1 as f64;
            // reset velocity
            self.state.ball.velocity.x = screen::BALL_START_VELOCITY.0 as f64;
            self.state.ball.velocity.y = screen::BALL_START_VELOCITY.1 as f64;
            // don't keep doing this!
            self.state.reset = false;
        }
        if buttons.left {
            self.state.p1_paddle.position.y += self.config.paddle_speed;
        } else if buttons.right {
            self.state.p1_paddle.position.y -= self.config.paddle_speed;
        }
        let ball_x = self.state.ball.position.x;
        let ball_y = self.state.ball.position.y;
        let ball_dx = self.state.ball.velocity.x;
        let ball_dy = self.state.ball.velocity.y;
        let p1_y = self.state.p1_paddle.position.y;
        let p2_y = self.state.p2_paddle.position.y;

        if ball_x < 0.0 {
            self.state.p1_score += 1;
            self.state.reset = true;
            return;
        } else if ball_x >= screen::GAME_SIZE.1 as f64 {
            self.state.p2_score += 1;
            self.state.reset = true;
            return;
        }

        // P2 AI:
        // AI clearly tries to hit ball in same spot of paddle each time:
        // roughly the size of the ball down from the top.
        let target = screen::BALL_SHAPE.1 as f64;
        let dest = ball_y - target;
        let speed = self.config.paddle_speed;
        // move towards ball constantly, limited by ball-speed.
        if (p2_y - dest).abs() <= speed {
            self.state.p2_paddle.position.y = dest;
        } else if p2_y < dest {
            self.state.p2_paddle.position.y += speed;
        } else {
            self.state.p2_paddle.position.y -= speed;
        }

        let ball = Rect::new(
            self.state.ball.position.x as i32,
            self.state.ball.position.y as i32,
            screen::BALL_SHAPE.0,
            screen::BALL_SHAPE.1,
        );
        let p1 = Rect::new(
            self.state.p1_paddle.position.x as i32,
            self.state.p1_paddle.position.y as i32,
            screen::PADDLE_SHAPE.0,
            screen::PADDLE_SHAPE.1,
        );
        let p2 = Rect::new(
            self.state.p2_paddle.position.x as i32,
            self.state.p2_paddle.position.y as i32,
            screen::PADDLE_SHAPE.0,
            screen::PADDLE_SHAPE.1,
        );

        // Now check bouncing.
        if ball_dx < 0.0 {
            // compare to p2
            if ball.intersects(&p2) {
                self.state.ball.velocity.x *= -1.0;
            }
        } else {
            // compare to p1
            if ball.intersects(&p1) {
                self.state.ball.velocity.x *= -1.0;
            }
        }

        // check floor/ceiling bounce:
        if ball_dy < 0.0 {
            // ceiling:
            if ball_y < (screen::TOP_FRAME_Y + screen::TOP_FRAME_H) as f64 {
                self.state.ball.velocity.y *= -1.0;
            }
        } else {
            if ball_y > screen::BOTTOM_FRAME_Y as f64 {
                self.state.ball.velocity.y *= -1.0;
            }
        }

        // Update ball:
        self.state.ball.integrate_mut(1.0);
    }
    fn draw(&self) -> Vec<toybox_core::graphics::Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(self.config.bg_color));

        // frame top:
        output.push(Drawable::rect(
            self.config.frame_color,
            0,
            screen::TOP_FRAME_Y,
            screen::GAME_SIZE.0,
            screen::TOP_FRAME_H,
        ));

        // frame bottom:
        output.push(Drawable::rect(
            self.config.frame_color,
            0,
            screen::BOTTOM_FRAME_Y,
            screen::GAME_SIZE.0,
            screen::BOTTOM_FRAME_H,
        ));

        if !self.state.reset {
            // ball:
            output.push(Drawable::rect(
                self.config.ball_color,
                self.state.ball.position.x as i32,
                self.state.ball.position.y as i32,
                screen::BALL_SHAPE.0,
                screen::BALL_SHAPE.1,
            ));
        }

        // p1:
        output.push(Drawable::rect(
            self.config.p1_color,
            self.state.p1_paddle.position.x as i32,
            self.state.p1_paddle.position.y as i32,
            screen::PADDLE_SHAPE.0,
            screen::PADDLE_SHAPE.1,
        ));

        // p2:
        output.push(Drawable::rect(
            self.config.p2_color,
            self.state.p2_paddle.position.x as i32,
            self.state.p2_paddle.position.y as i32,
            screen::PADDLE_SHAPE.0,
            screen::PADDLE_SHAPE.1,
        ));

        output
    }
    fn to_json(&self) -> String {
        serde_json::to_string(&self.state).expect("Should be no JSON Serialization Errors.")
    }
    fn copy(&self) -> Box<dyn toybox_core::State> {
        Box::new(self.clone())
    }
    fn query_json(
        &self,
        query: &str,
        args: &serde_json::Value,
    ) -> Result<String, toybox_core::QueryError> {
        Ok("TODO".to_string())
    }
}
