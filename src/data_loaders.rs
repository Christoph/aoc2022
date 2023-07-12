use std::fs::{read_to_string, File};
use std::io::BufReader;

pub fn read_file_as_string(path: &str) -> String {
    read_to_string(path).expect("Couldnt read file")
}

// for line in f.lines() {
//     let line = line.expect("Unable to read line");
//     println!("Line: {}", line);
// }
pub fn read_file_as_line_buffer(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("Couldnt read file");
    BufReader::new(file)
}
