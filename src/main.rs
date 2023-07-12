mod data_loaders;
mod day1;

fn main() {
    // let str_data = data_loaders::read_file_as_string("src/day1/input.txt");
    let buf_data = data_loaders::read_file_as_line_buffer("src/day1/input.txt");

    println!("Result day 1: {}", day1::day1_1(buf_data));
}
