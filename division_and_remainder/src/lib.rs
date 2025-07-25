pub fn divide(a: i32, b: i32) -> (i32, i32) {
    (a/b, a%b)
}

fn main() {
    let x = 9;
    let y = 4;
    let (division, remainder) = divide(x, y);
    println!(
        "{}/{}: division = {}, remainder = {}",
        x, y, division, remainder
    );
}