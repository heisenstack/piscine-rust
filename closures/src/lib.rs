pub fn first_fifty_even_square() -> Vec<i32> {
    (1..51).map(|x| (x * 2) * (x * 2)).collect()
}