use rand::prelude::*;

/// Apply a Fisher-Yates shuffle to a buffer
/// https://en.wikipedia.org/wiki/Fisher-Yates_shuffle
pub fn shuffle<T>(slice: &mut [T]) {
    let n = slice.len();
    if n == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    for i in 1..=(n - 1) {
        let j = rng.gen_range(0..=i);
        slice.swap(i, j);
    }
}
