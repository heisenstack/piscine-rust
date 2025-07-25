
pub fn to_url(s: &str) -> String {
    s.to_string().replace(" ", "%20")
}



fn main() {
    let s = "Hello, world!";
    println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
}