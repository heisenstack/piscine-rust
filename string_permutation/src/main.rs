use string_permutation::*;

fn main() {
    let word = "t ";
    let word1 = "t";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}