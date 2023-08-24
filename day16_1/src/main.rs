use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::{complete::take_while1, streaming::tag},
    character::complete::digit1,
    combinator::map,
    sequence::preceded,
    IResult,
};

impl Neighbor {
    fn value(&mut self, remaining_time: usize, open_valves: &HashSet<String>) {
        if open_valves.contains(&self.name) {
            self.value = 0;
        } else {
            self.value = (remaining_time - self.distance) * self.rate;
        }
    }
}

fn parse_rate(line: &str) -> IResult<&str, usize> {
    map(preceded(tag("rate="), digit1), |value: &str| {
        value.parse().expect("Not a number")
    })(line)
}

fn parse_valve(line: &str) -> IResult<&str, String> {
    map(preceded(tag("Valve "), parse_name), |v| v)(line)
}

fn parse_name(line: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c)),
        Into::into,
    )(line)
}

fn parse_line(line: &str) -> IResult<&str, Valve> {
    let (line, name) = parse_valve(line)?;
    let (line, _) = tag(" has flow ")(line)?;
    let (line, rate) = parse_rate(line)?;
    let (line, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(line)?;

    Ok((
        line,
        Valve {
            name,
            rate,
            tunnels: line.split(", ").map(|v| v.to_string()).collect(),
            neighbors: Vec::new(),
        },
    ))
}

fn main() {
    let valves: HashMap<String, Valve> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (_, valve) = parse_line(line).unwrap();
            (valve.name.clone(), valve)
        })
        .collect();
    // Init distance matrices
    for mut ele in valves {
        ele.1.detect_neighbors();
    }

    let mut time = 30;
    let mut pressure = 0;
    let mut current_valve = String::from("AA");
    let mut active_valves: HashSet<String> = HashSet::new();

    while time > 0 {
        // Get best move
        let valve = valves
            .get(current_valve)
            .expect("Valve should exist.{current_value} {valves:?}");
        let best_move = valve.best_move(time, &active_valves);

        // Execute move + 1 for activating if a best move exists
        // Stop the loop if no best move exists
        match best_move {
            Some(neighbor) => {
                time -= neighbor.distance + 1;
                pressure += neighbor.value;
                current_valve = neighbor.name;
                active_valves.insert(current_valve.clone());
            }
            None => time = 0,
        }
    }

    print!("Released pressure: {pressure}")
}

#[derive(Debug, Clone)]
struct Valve {
    rate: usize,
    name: String,
    tunnels: Vec<String>,
    neighbors: Vec<Neighbor>,
}

impl Valve {
    fn detect_neighbors(&mut self) {}

    fn best_move(
        &mut self,
        remaining_time: usize,
        open_valves: &HashSet<String>,
    ) -> Option<Neighbor> {
        let winner = self
            .neighbors
            .iter_mut()
            .map(|n| {
                n.value(remaining_time, open_valves);
                n
            })
            .max_by_key(|n| n.value)
            .expect("No neighbor");

        if winner.value == 0 {
            None
        } else {
            Some(winner.clone())
        }
    }
}

#[derive(Debug, Clone)]
struct Neighbor {
    name: String,
    rate: usize,
    distance: usize,
    value: usize,
}
