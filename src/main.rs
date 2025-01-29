mod array;
mod sort;

use macroquad::color::Color;
use macroquad::input::{self, KeyCode};
use macroquad::{shapes, window};
use sort::Sorts;

const WINDOW_WIDTH: f32 = 1500.0;
const WINDOW_HEIGHT: f32 = 800.0;
const BLOCK_WIDTH: f32 = 1.5;
const NUMBER_OF_BLOCKS: u16 = (WINDOW_WIDTH / BLOCK_WIDTH) as u16;
const BLOCK_HEIGHT_UNIT: f32 = WINDOW_HEIGHT / (NUMBER_OF_BLOCKS as f32);

#[macroquad::main("Visual Sort")]
async fn main() {
    window::request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut vec = array::gen_array(NUMBER_OF_BLOCKS);
    let mut sorted = false;
    let mut paused = true;

    loop {
        draw_array(&vec);

        if !sorted && !paused {
            sort::alg(&mut vec, Sorts::CocktailShakerSort).await;
            sorted = true;
        }

        if (sorted || paused) && input::is_key_released(KeyCode::R) {
            array::shuffle(&mut vec);
            sorted = false;
            paused = true;
        }

        if (sorted || paused)
            && (input::is_key_released(KeyCode::Space) || input::is_key_released(KeyCode::P))
        {
            paused = false;
        }

        window::next_frame().await;
    }
}

fn draw_array(vec: &[u16]) {
    let mut x = 0.0;

    for i in vec {
        let block_height = BLOCK_HEIGHT_UNIT * *i as f32;
        let y = WINDOW_HEIGHT - block_height;

        let hratio = block_height / WINDOW_HEIGHT;
        let colour = Color {
            r: hratio,
            g: 0.1,
            b: 1.0 - hratio,
            a: 1.0,
        };

        shapes::draw_rectangle(x, y, BLOCK_WIDTH, block_height, colour);

        x += BLOCK_WIDTH;
    }
}
