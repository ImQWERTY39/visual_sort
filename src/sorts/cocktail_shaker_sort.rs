pub async fn sort(vec: &mut [u16]) {
    let mut i = 0;
    let mut j = vec.len();
    let mut forward = true;

    let mut ptr = 0;

    while i <= j {
        if forward {
            while ptr < j - 1 {
                if vec[ptr] > vec[ptr + 1] {
                    super::swap(vec, ptr, ptr + 1).await;
                }

                ptr += 1;
            }

            forward = false;
            j -= 1;
        } else {
            while ptr > i {
                if vec[ptr] < vec[ptr - 1] {
                    super::swap(vec, ptr, ptr - 1).await;
                }

                ptr -= 1;
            }

            forward = true;
            i += 1;
        }
    }
}
