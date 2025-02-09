use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    let mut count = 0;
    let mut gap = vec.len() / 2;

    while gap > 0 {
        for i in gap..vec.len() {
            let key = vec[i].clone();
            let mut j = i;

            while j >= gap && vec[j - gap] > key {
                super::set(vec, j, vec[j - gap].clone(), &mut count).await;
                j -= gap;
            }

            super::set(vec, j, key, &mut count).await;
        }

        gap /= 2;
    }
}
