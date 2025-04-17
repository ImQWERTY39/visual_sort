mod array;
mod sorts;
mod ui;

use macroquad::input::{self, KeyCode};
use macroquad::window;
use sorts::Sorts;

const WINDOW_WIDTH: f32 = 1600.0;
const WINDOW_HEIGHT: f32 = 900.0;
const BLOCK_WIDTH: f32 = 0.125;
const NUMBER_OF_BLOCKS: u32 = (WINDOW_WIDTH / BLOCK_WIDTH) as u32;
const BLOCK_HEIGHT_UNIT: f32 = WINDOW_HEIGHT / (NUMBER_OF_BLOCKS as f32);

#[macroquad::main("Visual Sort")]
async fn main() {
    // window::request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    window::set_fullscreen(true);
    window::next_frame().await;

    let mut vec = array::gen_array(NUMBER_OF_BLOCKS);
    let mut paused = true;
    let mut alg = Sorts::default();

    loop {
        array::draw_array(&vec);
        ui::change_sort(&mut alg);

        if !paused {
            sorts::alg(&mut vec, alg).await;
        }

        paused = !(paused
            && (input::is_key_released(KeyCode::Space) || input::is_key_released(KeyCode::P)));
        window::next_frame().await;
    }
}
