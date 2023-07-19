use std::collections::HashMap;
use std::collections::VecDeque;
use {once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, PartialEq)]
struct UnloadingArea {
    size: usize,
    stacks: HashMap<usize, VecDeque<char>>,
}

impl UnloadingArea {
    fn new(n_stacks: usize) -> Self {
        let mut map: HashMap<usize, VecDeque<char>> = HashMap::new();
        for i in 0..n_stacks {
            map.insert(i, VecDeque::new());
        }
        UnloadingArea {
            stacks: map,
            size: n_stacks,
        }
    }

    fn cargo(&mut self, haystack: &str) {
        let movement = parse_movements(haystack);
        for _ in 0..movement.amount {
            let extract = self
                .stacks
                .get_mut(&movement.from)
                .expect("Stack is empty")
                .pop_front()
                .expect("No element");
            self.stacks
                .get_mut(&movement.to)
                .expect("Stack is empty")
                .push_front(extract)
        }
    }

    fn result(&mut self) -> String {
        let mut result = String::new();
        for i in 0..self.size {
            result.push(
                self.stacks
                    .get_mut(&i)
                    .expect("Couldnt find stack")
                    .pop_front()
                    .expect("Empty stack"),
            )
        }
        result
    }
}

#[derive(Debug, PartialEq)]
struct Movement {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_movements(haystack: &str) -> Movement {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\w+ (?<amount>\d+) \w+ (?<from>\d) \w+ (?<to>\d+)").unwrap());
    let re = RE.captures(haystack).unwrap();
    Movement {
        amount: re
            .name("amount")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .expect("No number!"),
        from: re
            .name("from")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .expect("No number!")
            - 1,
        to: re
            .name("to")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .expect("No number!")
            - 1,
    }
}

fn main() {
    let columns: usize = 9;
    let mut area = UnloadingArea::new(columns);
    let _: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            // map the current state
            if line.contains('[') {
                let chars = line.chars();
                for i in 0..columns {
                    let c = chars
                        .clone()
                        .nth(1 + i * (3 + 1))
                        .expect("Should be a character");

                    if c != ' ' {
                        area.stacks
                            .get_mut(&i)
                            .expect("Should be initialized")
                            .push_back(c);
                    }
                }
            } else if line.starts_with('m') {
                area.cargo(line);
            }
        })
        .collect();

    println!("{:#?}", area.result())
}
