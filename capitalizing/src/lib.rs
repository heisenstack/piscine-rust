pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    let mut capitalized = false;
    for i in input.chars() {
        if !capitalized {
            res += &i.to_string().to_uppercase();
            capitalized = true;
            continue;
        }
        res += &i.to_string();
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c)); 
                capitalize_next = false; 
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut res: Vec<String> = Vec::new();
    for i in input.chars() {
        if i.is_uppercase() {
            res.push(i.to_lowercase().to_string());
        }else {
            res.push(i.to_uppercase().to_string());
        }
    }
    res.join("")
}
