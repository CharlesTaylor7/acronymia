use crate::types::Range;
use rand::prelude::*;

/// Based on a
/// [Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_the_first_letters_of_a_word_in_English_language).
/// table of the relative frequency of each letter as the first letter in a word.
/// I'v copied those values, multiplying every percentage by 100 to get whole integers.
const DISTRIBUTION: [(char, usize); 26] = [
    ('S', 1100),
    ('C', 940),
    ('P', 770),
    ('D', 610),
    ('B', 600),
    ('R', 600),
    ('A', 570),
    ('M', 560),
    ('T', 500),
    ('F', 410),
    ('E', 390),
    ('I', 390),
    ('H', 370),
    ('G', 330),
    ('L', 310),
    ('U', 290),
    ('W', 270),
    ('O', 250),
    ('N', 220),
    ('V', 150),
    ('J', 110),
    ('K', 100),
    ('Q', 49),
    ('Y', 36),
    ('Z', 24),
    ('X', 5),
];

const SUM_OF_WEIGHTS: usize = {
    let mut total: usize = 0;
    let mut i = 0;
    // iter() is not allowed at compile time, but basic loops are!
    while i < DISTRIBUTION.len() {
        total += DISTRIBUTION[i].1;
        i += 1;
    }
    total
};

/// Makes a random selection of n letters, using relative dictionary frequencies.
/// Not necessarily pronounceable, so it's an initialism, not an acronym.
pub fn random_initialism(range: &Range<usize>) -> String {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(range.min..=range.max);
    (0..length).map(|_| random_letter()).collect()
}

fn random_letter() -> char {
    let mut rng = rand::thread_rng();
    let mut value = rng.gen_range(0..SUM_OF_WEIGHTS);

    // The basic idea is:
    // If 0 <= value < weight_0, then return the first letter in the array.
    // if weight_0 <= value < weight_0 + weight_1 then return the second letter and so on.
    //
    // But we can optimize by subtracting each weight if there's no match yet.
    // This works out to the same without keeping a running subtally of weights.
    for (letter, weight) in DISTRIBUTION {
        if value < weight {
            return letter;
        } else {
            value -= weight;
        }
    }
    panic!("impossible")
}
