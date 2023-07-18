use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let letters = (b'a'..=b'z').chain(b'A'..=b'Z').map(|c| c as char);

    let result = include_str!("input.txt")
        .lines()
        .map(|v| {
            let (f, s) = v.split_at(v.len() / 2);
            let hsf: HashSet<char> = HashSet::from_iter(f.chars());
            let hss: HashSet<char> = HashSet::from_iter(s.chars());

            hsf.intersection(&hss)
                .copied()
                .map(|v| letters.clone().position(|f| f == v).unwrap() + 1)
                .sum::<usize>()

            // intersection.collect::<Vec<_>>()
        })
        .sum::<usize>();

    println!("{:#?}", result)
}
