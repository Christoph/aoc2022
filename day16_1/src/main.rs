use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::{
        complete::take_while1,
        streaming::{tag, take_until1},
    },
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    Finish, IResult,
};

#[derive(Debug)]
struct Valve {
    rate: usize,
    name: String,
    tunnels: Vec<String>,
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
}
