pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_char: Vec<_> = s1.chars().collect();
    let mut s2_char: Vec<_> = s2.chars().collect();
    s1_char.sort();
    s2_char.sort();
    s1_char == s2_char
}
