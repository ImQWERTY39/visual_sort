use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    helper(vec, 0, vec.len(), true).await;
}

#[async_recursion::async_recursion]
async fn helper(vec: &mut [Rectangle], low: usize, n: usize, asc: bool) {
    let mut nf_count = 0;
    if n <= 1 {
        return;
    }

    let k = n / 2;
    helper(vec, low, k, true).await;
    helper(vec, low + k, k, false).await;
    merge(vec, low, n, asc, &mut nf_count).await;
}

#[async_recursion::async_recursion]
async fn merge(vec: &mut [Rectangle], low: usize, n: usize, asc: bool, nf_count: &mut u8) {
    if n <= 1 {
        return;
    }

    let k = n / 2;
    for i in low..(low + k) {
        if (asc && vec[i] > vec[i + k]) || (!asc && vec[i] < vec[i + k]) {
            super::swap(vec, i, i + k, nf_count).await;
        }
    }

    merge(vec, low, k, asc, nf_count).await;
    merge(vec, low + k, k, asc, nf_count).await;
}
