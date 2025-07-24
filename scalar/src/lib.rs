pub fn sum(x: u8, y: u8) -> u8 {
    x + y
}

pub fn diff(x: i16, y: i16) -> i16 {
    x - y
}

pub fn pro(x: i8, y: i8) -> i8 {
    x * y
}

pub fn quo(x: i32, y: i32) -> i32 {
    x / y
}

pub fn rem(x: i32, y: i32) -> i32 {
    x % y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
}
