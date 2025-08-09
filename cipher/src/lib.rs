#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut holder = String::new();
    for ch in original.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_lowercase() { b'z' } else { b'Z' };
            let offset = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
            let new = base - ch as u8 + offset;
            holder.push(new as char);
        } else {
            holder.push(ch)
        }
    }
    if holder != ciphered {
        return Err(CipherError {
            expected: format!("{}", holder),
        });
    }
    Ok(())
}