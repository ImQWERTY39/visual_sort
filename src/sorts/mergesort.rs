use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    if vec.len() <= 1 {
        return;
    }

    let mut temp = vec.to_vec();
    let mut stack = vec![(0, vec.len())];

    let mut subarrays = vec![];
    while let Some((start, end)) = stack.pop() {
        if end - start <= 1 {
            continue;
        }

        let mid = (start + end) / 2;
        stack.push((start, mid));
        stack.push((mid, end));

        subarrays.push((start, mid, end));
    }

    while let Some((start, mid, end)) = subarrays.pop() {
        merge(vec, &mut temp, start, mid, end).await;
    }
}

async fn merge(
    vec: &mut [Rectangle],
    temp: &mut [Rectangle],
    start: usize,
    mid: usize,
    end: usize,
) {
    let mut count = 0;

    let mut left = start;
    let mut right = mid;
    let mut k = start;

    while left < mid && right < end {
        if vec[left] <= vec[right] {
            temp[k] = vec[left].clone();
            left += 1;
        } else {
            temp[k] = vec[right].clone();
            right += 1;
        }
        k += 1;
    }

    while left < mid {
        temp[k] = vec[left].clone();
        left += 1;
        k += 1;
    }

    while right < end {
        temp[k] = vec[right].clone();
        right += 1;
        k += 1;
    }

    for (i, rect) in temp.iter().enumerate().take(end).skip(start) {
        super::set(vec, i, rect.clone(), &mut count).await;
    }
}
