use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Couldnt read file");
    let buf_data = BufReader::new(file);
    println!("{}", day1_1(buf_data));
}

pub fn day1_1(buf_data: BufReader<File>) -> i32 {
    let mut max_value: i32 = 0;
    let mut current_value: i32 = 0;

    for line in buf_data.lines() {
        let line = line.expect("Unable to read line");
        let parsed = line.parse::<i32>();

        match parsed {
            Ok(result) => {
                current_value += result;
                if current_value > max_value {
                    max_value = current_value;
                }
            }
            Err(_) => {
                current_value = 0;
            }
        }
    }

    max_value
}
