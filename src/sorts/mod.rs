use std::time::{Duration, Instant};

use crate::{array, draw_array};
use macroquad::{audio, window};

mod cocktail_shaker_sort;
mod insertion_sort;
mod mergesort;
mod quicksort;
mod selection_sort;

pub enum Sorts {
    SelectionSort,
    InsertionSort,
    CocktailShakerSort,
    QuickSort,
    MergeSort,
}

pub async fn alg(vec: &mut [u16], alg: Sorts) {
    shuffle_before_start(vec).await;

    let end_audio = audio::load_sound("./assets/end.wav").await.unwrap();
    audio::play_sound_once(&end_audio);

    match alg {
        Sorts::SelectionSort => selection_sort::sort(vec).await,
        Sorts::InsertionSort => insertion_sort::sort(vec).await,
        Sorts::CocktailShakerSort => cocktail_shaker_sort::sort(vec).await,
        Sorts::QuickSort => quicksort::sort(vec).await,
        Sorts::MergeSort => mergesort::sort(vec).await,
    }

    audio::stop_sound(&end_audio);
}

async fn shuffle_before_start(vec: &mut [u16]) {
    let start_audio = audio::load_sound("./assets/start.wav").await.unwrap();
    audio::play_sound_once(&start_audio);

    let now = Instant::now();
    while now.elapsed() < Duration::from_millis(8433) {
        array::shuffle(vec);

        draw_array(vec);
        window::next_frame().await;
    }

    let now = Instant::now();
    while now.elapsed() < Duration::from_millis(200) {}
}

async fn swap(vec: &mut [u16], i: usize, j: usize) {
    vec.swap(i, j);

    draw_array(vec);
    window::next_frame().await;
}

async fn set(vec: &mut [u16], i: usize, value: u16) {
    vec[i] = value;

    draw_array(vec);
    window::next_frame().await;
}
