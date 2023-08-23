use std::collections::HashSet;

fn main() {
    let mut sand_source = (500, 0);
    let mut blocked_spaces: HashSet<(usize, usize)> = HashSet::default();
    let mut sand_counter = 0;

    include_str!("input.txt").lines().for_each(|line| {
        let positions: Vec<_> = line
            .split(" -> ")
            .map(|position| {
                let temp = position
                    .split(',')
                    .map(|coord| coord.parse().unwrap())
                    .collect::<Vec<usize>>();
            })
            .collect();
        println!("{positions:?}")
    })
}
