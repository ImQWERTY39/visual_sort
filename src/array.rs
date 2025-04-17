use crate::{BLOCK_HEIGHT_UNIT, BLOCK_WIDTH, WINDOW_HEIGHT};
use macroquad::{color::Color, shapes};
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Rectangle {
    pub value: u32,
    pub y: f32,
    pub height: f32,
    pub colour: Color,
}

pub fn gen_array(size: u32) -> Vec<Rectangle> {
    (1..=size).map(Rectangle::new).collect()
}

pub fn shuffle(vec: &mut [Rectangle]) {
    vec.shuffle(&mut rand::thread_rng());
}

pub fn draw_array(vec: &[Rectangle]) {
    let mut x = 0.0;

    for rect in vec {
        shapes::draw_rectangle(x, rect.y, BLOCK_WIDTH, rect.height, rect.colour);
        x += BLOCK_WIDTH;
    }
}

impl Rectangle {
    pub fn new(value: u32) -> Self {
        let height = BLOCK_HEIGHT_UNIT * value as f32;
        let y = WINDOW_HEIGHT - height;
        let hratio = height / WINDOW_HEIGHT;
        let colour = Color {
            r: hratio,
            g: 0.1,
            b: 1.0 - hratio,
            a: 1.0,
        };

        Rectangle {
            value,
            y,
            height,
            colour,
        }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Rectangle {}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
