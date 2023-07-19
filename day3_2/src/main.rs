use im::HashSet;
use itertools::Itertools;

fn main() {
    let letters = (b'a'..=b'z').chain(b'A'..=b'Z').map(|c| c as char);

    let result: usize = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .reduce(|x, y| x.intersection(y))
                .expect("Not a group of 3")
                .iter()
                .copied()
                .map(|v| letters.clone().position(|f| f == v).unwrap() + 1)
                .next()
                .expect("There should be at least one.")
        })
        .sum();

    println!("{:#?}", result)
}
