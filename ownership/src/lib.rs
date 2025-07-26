pub fn first_subword( s: String) -> String {
    let mut res = String::new();
    let mut found_upper_case = false;
    for c in s.chars() {
        if c == '_' {
            break;
        } 
        if  c.is_uppercase() {
            if found_upper_case || res.chars().count() != 0 {
                // println!("{c}");
                break;
            } else {
                found_upper_case = true;
                res.push(c);
                continue;
                // break;
            }
        }  
        res.push(c);
    }
    res
}

fn main() {
    let s1 = "helloWorld";
    let s2 = "snake_case";
    let s3 = "CamelCase";
    let s4 = "just";

    println!("first_subword({}) = {}", s1, first_subword(s1.to_owned()));
    println!("first_subword({}) = {}", s2, first_subword(s2.to_owned()));
    println!("first_subword({}) = {}", s3, first_subword(s3.to_owned()));
    println!("first_subword({}) = {}", s4, first_subword(s4.to_owned()));
}