use std::time;

use crate::{array, draw_array};
use macroquad::{audio, window};
use rand::Rng;

pub enum Sorts {
    SelectionSort,
    QuickSort,
}

pub async fn alg(vec: &mut [u16], alg: Sorts) {
    shuffle_before_start(vec).await;

    let end_audio = audio::load_sound("./assets/end.wav").await.unwrap();
    audio::play_sound_once(&end_audio);

    match alg {
        Sorts::SelectionSort => selection_sort(vec).await,
        Sorts::QuickSort => quicksort(vec).await,
    }

    audio::stop_sound(&end_audio);
}

async fn selection_sort(vec: &mut [u16]) {
    for i in 0..vec.len() {
        let mut smallest_idx = i;

        for j in (smallest_idx + 1)..vec.len() {
            if vec[j] < vec[smallest_idx] {
                smallest_idx = j;
            }
        }

        swap(vec, i, smallest_idx).await;
    }
}

async fn quicksort(vec: &mut [u16]) {
    let mut indexes = Vec::new();
    indexes.push((0, vec.len() - 1));

    while let Some((start, end)) = indexes.pop() {
        if start >= end {
            continue;
        }

        let pivot_idx = rand::thread_rng().gen_range(start..=end);
        let pivot = vec[pivot_idx];

        swap(vec, pivot_idx, end).await;

        let mut i = start;
        let mut j = end - 1;

        'mainloop: while i <= j && i < end {
            while vec[i] < pivot && i <= j {
                i += 1;
            }

            while vec[j] >= pivot && i <= j {
                if j > 0 {
                    j -= 1;
                } else {
                    break 'mainloop;
                }
            }

            if i < j {
                swap(vec, i, j).await;
            }
        }

        swap(vec, i, end).await;

        indexes.push((i + 1, end));
        indexes.push((start, i.checked_sub(1).unwrap_or(start)));
    }
}

async fn swap(vec: &mut [u16], i: usize, j: usize) {
    vec.swap(i, j);

    draw_array(vec);
    window::next_frame().await;
}

async fn shuffle_before_start(vec: &mut [u16]) {
    let start_audio = audio::load_sound("./assets/start.wav").await.unwrap();
    audio::play_sound_once(&start_audio);

    let now = time::Instant::now();
    while now.elapsed() < time::Duration::from_millis(8433) {
        array::shuffle(vec);

        draw_array(vec);
        window::next_frame().await;
    }

    let now = time::Instant::now();
    while now.elapsed() < time::Duration::from_millis(200) {}
}
