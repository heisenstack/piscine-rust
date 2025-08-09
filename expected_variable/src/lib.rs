mod edit_distance;
use crate::edit_distance::*;

pub fn expected_variable(source: &str, target: &str) -> Option<String> {
    if !is_camel_case(source) && !is_snake_case(source) {
        return None;
    }

    let distance = edit_distance(&source.to_lowercase(), &target.to_lowercase());
    let target_length = target.len();

    let similarity_score = 100 - ((distance as u32 * 100) / target_length as u32);

    if similarity_score < 50 {
        return None;
    }

    Some(format!("{}%", similarity_score))
}

fn is_camel_case(identifier: &str) -> bool {
    let chars: Vec<char> = identifier.chars().collect();

    for i in 1..chars.len() {
        if chars[i].is_uppercase() {
            return true;
        }
        if chars[i].is_whitespace() {
            return false;
        }
    }

    false
}

fn is_snake_case(identifier: &str) -> bool {
    identifier.contains('_')
}
