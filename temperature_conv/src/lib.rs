pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32 as f64) * (9 as f64 / 5 as f64)
}
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9 as f64 / 5 as f64)) + 32 as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(137 as f64);
        assert_eq!(result, 189 as f64);
    }
}
