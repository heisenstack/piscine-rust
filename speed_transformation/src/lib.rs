pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    (km_h / 60 as f64) * 1000 as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = km_per_hour_to_meters_per_second(60 as f64);
        assert_eq!(result, 1000 as f64);
    }
}
