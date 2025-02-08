use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    let max = vec.iter().max().unwrap().value;
    let mut exp = 1;

    while max / exp > 0 {
        sort_digit(vec, exp).await;
        exp *= 10;
    }
}

async fn sort_digit(vec: &mut [Rectangle], exp: u32) {
    let mut count = 0;

    let mut output = vec.to_vec();
    let mut digit_count = [0; 10];

    for num in vec.iter().map(|x| x.value) {
        let digit = (num / exp % 10) as usize;
        digit_count[digit] += 1;
    }

    for i in 1..10 {
        digit_count[i] += digit_count[i - 1];
    }

    for (rect, num) in vec.iter().rev().map(|x| (x, x.value)) {
        let digit = (num / exp % 10) as usize;
        digit_count[digit] -= 1;
        output[digit_count[digit]] = rect.clone();
    }

    for (idx, value) in output.drain(..).enumerate() {
        super::set(vec, idx, value, &mut count).await;
    }
}
