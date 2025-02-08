use crate::array::Rectangle;

pub async fn sort(vec: &mut [Rectangle]) {
    let mut count = 0;
    let mut i = 0;
    let mut j = vec.len();
    let mut forward = true;

    let mut ptr = 0;

    while i <= j {
        if forward {
            while ptr < j - 1 {
                if vec[ptr] > vec[ptr + 1] {
                    super::swap(vec, ptr, ptr + 1, &mut count).await;
                }

                ptr += 1;
            }

            forward = false;
            j -= 1;
        } else {
            while ptr > i {
                if vec[ptr] < vec[ptr - 1] {
                    super::swap(vec, ptr, ptr - 1, &mut count).await;
                }

                ptr -= 1;
            }

            forward = true;
            i += 1;
        }
    }
}
