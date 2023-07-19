use itertools::Itertools;
use std::collections::HashMap;
fn main() {
    let mut counter = 0;
    let result: Vec<_> = include_str!("input.txt")
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .take_while(|tuple| {
            counter += 1;
            let mut hash = HashMap::new();
            hash.insert(tuple.0, true);
            hash.insert(tuple.1, true);
            hash.insert(tuple.2, true);
            hash.insert(tuple.3, true);
            hash.keys().len() < 4
        })
        .collect();

    println!("{:?}, {}", result, counter + 3)
}
