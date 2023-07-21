use std::collections::HashMap;
fn main() {
    let data: &str = include_str!("input.txt");
    let step_size = 14;
    let mut counter = step_size - 1;

    for i in 0..data.len() - step_size {
        counter += 1;
        let mut hash = HashMap::new();
        let slice = &data[i..i + step_size];

        for c in slice.chars() {
            hash.insert(c, true);
        }

        if hash.len() == step_size {
            println!("result: {:?}", counter);
            break;
        }
    }
}
