use rand::prelude::*;

/// Apply a [Fisher-Yates shuffle](https://en.wikipedia.org/wiki/Fisher-Yates_shuffle) to a buffer
pub fn shuffle<T>(slice: &mut [T]) {
    let n = slice.len();
    if n < 2 {
        return;
    }

    let mut rng = rand::thread_rng();
    for i in 1..=(n - 1) {
        let j = rng.gen_range(0..=i);
        slice.swap(i, j);
    }
}
