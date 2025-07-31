use std::path::Path;
use std::fs::File;
use std::io::Write;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = File::open(path).unwrap_or_else(|_|
        File::create(path).expect("cannot create file")
    );
    file.write_all(content.as_bytes()).expect("cannot write to file");
}
