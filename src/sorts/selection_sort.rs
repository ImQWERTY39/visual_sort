use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    let mut count = 0;
    for i in 0..vec.len() {
        let mut smallest_idx = i;

        for j in (smallest_idx + 1)..vec.len() {
            if vec[j] < vec[smallest_idx] {
                smallest_idx = j;
            }
        }

        super::swap(vec, i, smallest_idx, &mut count).await;
    }
}
