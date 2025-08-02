use std::fs::File;
pub fn open_file(s: &str) -> File {
    File::open(s.to_string()).unwrap()
}