use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let max:i32 = 0;
    if let Some(max) = h.values().max() {
        return *max;
    }
    max
}