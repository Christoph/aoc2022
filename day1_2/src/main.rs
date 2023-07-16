use itertools::Itertools;

fn main() {
    let result: i32 = include_str!("input.txt")
        .lines()
        .map(|x| x.parse::<i32>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("Result day1 part 2: {}", result)
}
