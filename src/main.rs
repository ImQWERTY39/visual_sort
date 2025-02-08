mod array;
mod sorts;

use macroquad::input::{is_key_released, KeyCode};
use macroquad::window;

use sorts::Sorts;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const BLOCK_WIDTH: f32 = 0.125;
const NUMBER_OF_BLOCKS: u32 = (WINDOW_WIDTH / BLOCK_WIDTH) as u32;
const BLOCK_HEIGHT_UNIT: f32 = WINDOW_HEIGHT / (NUMBER_OF_BLOCKS as f32);

#[macroquad::main("Visual Sort")]
async fn main() {
    window::request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut vec = array::gen_array(NUMBER_OF_BLOCKS);
    let mut paused = true;

    loop {
        array::draw_array(&vec);

        if !paused {
            sorts::alg(&mut vec, Sorts::MergeSort).await;
            paused = true;
        }

        paused = !(paused && (is_key_released(KeyCode::Space) || is_key_released(KeyCode::P)));
        window::next_frame().await;
    }
}
