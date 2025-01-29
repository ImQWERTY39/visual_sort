use rand::seq::SliceRandom;

pub fn gen_array(size: u16) -> Vec<u16> {
    (1..=size).collect()
}

pub fn shuffle(vec: &mut [u16]) {
    vec.shuffle(&mut rand::thread_rng());
}
