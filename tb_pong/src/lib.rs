#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate schemars;
#[macro_use]
extern crate lazy_static;

pub mod pong;
pub mod types;

pub use types::PongConfig;

/// Use 2d types shared with tb_breakout.
pub use toybox_core::body2d::Body2D;
pub use toybox_core::vec2d::Vec2D;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
