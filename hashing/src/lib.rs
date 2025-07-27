pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    (sum as f64) / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted_list: Vec<i32> = list.to_vec();
    sorted_list.sort_unstable();

    let len = sorted_list.len();
    let mid_index = len / 2;

    if len % 2 != 0 {
        sorted_list[mid_index]
    } else {
        let val1 = sorted_list[mid_index - 1];
        let val2 = sorted_list[mid_index];
        (val1 + val2) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
 let mut frequency_map = std::collections::HashMap::new();
    for &num in list {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    frequency_map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap_or(0) 
}