use std::time::{Duration, Instant};

use crate::{
    array::{self, Rectangle},
    BLOCK_WIDTH, WINDOW_HEIGHT,
};
use macroquad::audio;
use macroquad::color::Color;
use macroquad::shapes;
use macroquad::window;

const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
const THRESHOLD: u8 = 16;

mod cocktail_shaker_sort;
mod insertion_sort;
mod mergesort;
mod quicksort;
mod radix_sort;
mod selection_sort;

#[allow(unused)]
pub enum Sorts {
    SelectionSort,
    InsertionSort,
    CocktailShakerSort,
    QuickSort,
    MergeSort,
    RadixSort,
}

pub async fn alg(vec: &mut [Rectangle], alg: Sorts) {
    shuffle_before_start(vec).await;

    let end_audio = audio::load_sound("./assets/end.wav").await.unwrap();
    audio::play_sound_once(&end_audio);

    match alg {
        Sorts::SelectionSort => selection_sort::sort(vec).await,
        Sorts::InsertionSort => insertion_sort::sort(vec).await,
        Sorts::CocktailShakerSort => cocktail_shaker_sort::sort(vec).await,
        Sorts::QuickSort => quicksort::sort(vec).await,
        Sorts::MergeSort => mergesort::sort(vec).await,
        Sorts::RadixSort => radix_sort::sort(vec).await,
    }

    audio::stop_sound(&end_audio);
}

async fn shuffle_before_start(vec: &mut [Rectangle]) {
    let start_audio = audio::load_sound("./assets/start.wav").await.unwrap();
    audio::play_sound_once(&start_audio);

    let now = Instant::now();
    while now.elapsed() < Duration::from_millis(8433) {
        array::shuffle(vec);

        array::draw_array(vec);
        window::next_frame().await;
    }

    let now = Instant::now();
    while now.elapsed() < Duration::from_millis(200) {}
}

async fn swap(vec: &mut [Rectangle], i: usize, j: usize, count: &mut u8) {
    let rect1 = vec[i].clone();
    let rect2 = vec[j].clone();

    let ix = i as f32 * BLOCK_WIDTH;
    let jx = j as f32 * BLOCK_WIDTH;

    shapes::draw_rectangle(ix, 0.0, BLOCK_WIDTH, WINDOW_HEIGHT, BLACK);
    shapes::draw_rectangle(jx, 0.0, BLOCK_WIDTH, WINDOW_HEIGHT, BLACK);

    shapes::draw_rectangle(jx, rect1.y, BLOCK_WIDTH, rect1.height, rect1.colour);
    shapes::draw_rectangle(ix, rect2.y, BLOCK_WIDTH, rect2.height, rect2.colour);

    vec.swap(i, j);

    if *count >= THRESHOLD {
        array::draw_array(vec);
        window::next_frame().await;
        *count = 0;
    } else {
        *count += 1;
    }
}

async fn set(vec: &mut [Rectangle], i: usize, value: Rectangle, count: &mut u8) {
    shapes::draw_rectangle(
        i as f32 * BLOCK_WIDTH,
        value.y,
        BLOCK_WIDTH,
        value.height,
        value.colour,
    );
    vec[i] = value;

    if *count >= THRESHOLD {
        array::draw_array(vec);
        window::next_frame().await;
        *count = 0;
    } else {
        *count += 1;
    }
}
