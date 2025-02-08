use rand::Rng;

use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    let mut count = 0;
    let mut random = rand::thread_rng();
    let mut indexes = vec![(0, vec.len() - 1)];

    while let Some((start, end)) = indexes.pop() {
        if start >= end {
            continue;
        }

        let pivot_idx = random.gen_range(start..=end);
        let pivot = vec[pivot_idx].clone();
        super::swap(vec, pivot_idx, end, &mut count).await;

        let mut i = start;
        let mut j = end - 1;

        'mainloop: while i <= j {
            while vec[i] < pivot && i <= j {
                i += 1;
            }

            while vec[j] > pivot && j >= i {
                if j == 0 {
                    break 'mainloop;
                }

                j -= 1;
            }

            if i < j {
                super::swap(vec, i, j, &mut count).await;
            }
        }

        super::swap(vec, i, end, &mut count).await;

        indexes.push((i + 1, end));
        indexes.push((start, i.checked_sub(1).unwrap_or(start)));
    }
}
