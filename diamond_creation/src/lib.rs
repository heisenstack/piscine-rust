pub fn get_diamond(c: char) -> Vec<String> {
    let target_index = (c as u8) - b'A'; 
    let mut diamond: Vec<String> = Vec::new();

    for i in 0..=target_index {
        let current_char = (b'A' + i) as char;
        let outer_spaces = (target_index - i) as usize;
        let inner_spaces = if i == 0 { 0 } else { (2 * i - 1) as usize };

        let mut line = String::new();

        line.push_str(&" ".repeat(outer_spaces));
        line.push(current_char);
        if inner_spaces > 0 {
            line.push_str(&" ".repeat(inner_spaces));
            line.push(current_char);
        }
        line.push_str(&" ".repeat(outer_spaces));

        diamond.push(line);
    }

    for i in (0..target_index as usize).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}
