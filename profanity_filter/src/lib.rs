pub fn check_ms(message: &str) -> Result<&str, &str> {
    match message.contains("stupid") || message.is_empty() {
        true => Err("ERROR: illegal"),
        _ => Ok(message)
    }
}