pub fn km_per_hour_to_meters_per_second(km_h: u64) -> u64 {
    (km_h / 60 ) * 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = km_per_hour_to_meters_per_second(60);
        assert_eq!(result, 1000);
    }
}
