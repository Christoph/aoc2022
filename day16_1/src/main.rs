use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::{complete::take_while1, streaming::tag},
    character::complete::digit1,
    combinator::map,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Valve {
    rate: usize,
    name: String,
    tunnels: Vec<String>,
    neighbors: Vec<Neighbor>,
}

impl Valve {
    fn detect_neighbors(&mut self) {}

    fn best_move(&mut self, remaining_time: usize, open_valves: &HashSet<String>) -> Neighbor {
        let winner = self
            .neighbors
            .iter_mut()
            .map(|n| {
                n.value(remaining_time, open_valves);
                n
            })
            .max_by_key(|n| n.value)
            .expect("No neighbor");

        winner.clone()
    }
}

#[derive(Debug, Clone)]
struct Neighbor {
    name: String,
    rate: usize,
    distance: usize,
    value: usize,
}

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
    let mut time = 30;
    let mut pressure = 0;

    // Get distances from a sttart point to all other points
    // Multiply the remaining timesteps (time - distance) with the pressure for each point
    // 1st step value
    // do this recursivley until time is out or all are open and sum all
}
