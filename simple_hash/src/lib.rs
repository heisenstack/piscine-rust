use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut my_map = HashMap::new();
    for word in words {
        *my_map.entry(word.clone()).or_insert(0) += 1 as usize;
    }  
    my_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}