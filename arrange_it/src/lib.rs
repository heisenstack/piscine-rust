use std::collections::HashMap;

pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = Vec::new();
    let mut my_map = HashMap::new();
    let mut num = 0;
    let arr: Vec<_> = phrase.split(" ").collect();
    for word in arr {
        let mut new_word : String = String::new();
        for w in word.chars() {
            if w.is_digit(10) {
                num = w.to_string().parse().expect("Error Parsing String");
                continue;
            }
            new_word.push(w)
        }
        my_map.insert(num, new_word.clone());
    }
    let mut i = 1;
    let max_key = my_map.keys().max().copied().unwrap_or(0);
    while i <= max_key { 
        if let Some(s) = my_map.get(&i) {
            res.push(s.clone()); 
        }
        i += 1;
    }
    res.join(" ")

}

fn main() {
    println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
}
