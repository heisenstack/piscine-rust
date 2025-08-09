use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;


pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let mut f = OpenOptions::new()
   .create(true)
   .append(true)
   .open(path)
   .unwrap();

    f.write_all(content.as_bytes()).unwrap()

}