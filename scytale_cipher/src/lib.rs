pub fn scytale_cipher(message: &str, num_columns: u32) -> String {
    if message.len() == 0 || message.len() == num_columns as usize {
        return message.to_string();
    }

    let num_rows = (message.len() as f64 / num_columns as f64).ceil() as usize;
    let mut grid: Vec<Vec<char>> = vec![vec![' '; num_columns as usize]; num_rows];
    let mut cipher_text: String = String::new();

    let mut row: usize = 0;
    let mut col: usize = 0;

    for character in message.chars() {
        if col == num_columns as usize {
            col = 0;
            row += 1;
        }
        grid[row][col] = character;
        col += 1;
    }

    for col in 0..num_columns as usize {
        for row in 0..num_rows {
            cipher_text.push(grid[row][col]);
        }
    }

    cipher_text.trim().to_owned()
}
