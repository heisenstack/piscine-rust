pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|num_str| {
            let mut num_f32 = num_str.trim_end_matches('k').parse::<f32>().unwrap_or(0.0);
            if num_str.ends_with('k') {
                num_f32 *= 1000.0;
            }
            Box::new(num_f32 as u32)
        })
        .collect()
}
pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}
