use toybox_core::graphics::Color;
use toybox_core::graphics::FixedSpriteData;

const HAIR_COLOR: (u8, u8, u8) = (105, 105, 15);
const FACE_COLOR: (u8, u8, u8) = (228, 111, 111);
const SHIRT_COLOR: (u8, u8, u8) = (92, 186, 92);
const PANTS_COLOR: (u8, u8, u8) = (53, 95, 24);

fn load_sprite(data: &[&str]) -> FixedSpriteData {
    let off_color = Color::invisible();
    let mut pixels: Vec<Vec<Color>> = Vec::new();
    for line in data {
        let mut pixel_row = Vec::new();
        for ch in line.chars() {
            pixel_row.push(match ch {
                '1' => Color::from(&HAIR_COLOR),
                '2' => Color::from(&FACE_COLOR),
                '3' => Color::from(&SHIRT_COLOR),
                '4' => Color::from(&PANTS_COLOR),
                '.' => off_color,
                _ => panic!(
                    "Cannot construct pixel from {}, expected one of (1,2,3,4,.)",
                    ch
                ),
            });
        }
        pixels.push(pixel_row)
    }
    let width = pixels[0].len();
    debug_assert!(pixels.iter().all(|row| row.len() == width));
    FixedSpriteData::new(pixels)
}

/// Parse digit sprites text files, splitting on blank lines.
pub fn load_harry_sprites(data: &str) -> Vec<FixedSpriteData> {
    let mut sprites = Vec::new();
    let mut current = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() && current.len() > 0 {
            sprites.push(load_sprite(&current));
            current.clear();
        } else {
            current.push(line);
        }
    }
    if current.len() > 0 {
        sprites.push(load_sprite(&current));
    }

    sprites.into_iter().rev().collect()
}
