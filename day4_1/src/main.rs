use itertools::Itertools;

fn main() {
    let result = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|assigment| {
                    assigment
                        .split('-')
                        .map(|entry| entry.parse::<u32>().expect("This should be a number"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("There should be a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("There should be two ranges")
        })
        .filter(|(first, second)| {
            first.contains(second.start()) && first.contains(second.end())
                || second.contains(first.start()) && second.contains(first.end())
        })
        .count();

    println!("{:#?}", result)
}
