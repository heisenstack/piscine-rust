pub fn rev_str(input: &str) -> String {
    let mut result = String::new();

    for i in input.chars().rev() {
        result.push(i);
    }
    result
}