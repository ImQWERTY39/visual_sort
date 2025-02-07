pub async fn sort(vec: &mut [u16]) {
    for i in 0..vec.len() {
        for j in (1..i).rev() {
            if vec[j] > vec[j - 1] {
                break;
            }

            super::swap(vec, j, j - 1).await;
        }
    }
}
