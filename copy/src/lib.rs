pub fn nbr_function(c: i32) -> (i32, f64, f64) {
(c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut exp_str = String::new();
    for c in a.chars() {
            if let Some(digit_value) = c.to_digit(10) {
                exp_str = format!("{} {}", exp_str, (digit_value as f64).exp().to_string())
            }
    }
    (a, exp_str.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut exp: Vec<f64> = Vec::new();
    for c in &b {
        exp.push((c.abs()as f64).ln())
    }
    (b, exp)
}

fn main() {
    let a = "1 2 4 5 6".to_owned();
    let b = vec![1, 2, 4, 5];
    let c = 0;

    println!("{:?}", nbr_function(c));
    println!("{:?}", vec_function(b));
    println!("{:?}", str_function(a));
}