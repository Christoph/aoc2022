use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{complete::digit1, streaming::alpha1},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn parse_line(line: &str) -> IResult<&str, Valve> {
    let (line, _) = tag("Valve ")(line)?;
    let (line, name) = alpha1(line)?;
    let (line, _) = tag(" has flow rate=")(line)?;
    let (line, rate) = digit1(line)?;
    let (line, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(line)?;

    Ok((
        line,
        Valve {
            name: String::from(name),
            rate: rate.parse().expect("Couldnt parse number"),
            tunnels: line.split(", ").map(|v| v.to_string()).collect(),
            neighbors: Vec::new(),
        },
    ))
}

fn main() {
    let start_time = Instant::now();
    let mut valves: HashMap<String, Valve> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (_, valve) = parse_line(line).unwrap();
            (valve.name.clone(), valve)
        })
        .collect();
    let lookup_map = valves.clone();
    valves
        .iter_mut()
        .for_each(|ele| ele.1.detect_neighbors(&lookup_map));

    let mut max_distance = 3;
    let mut result = 1;
    let mut previous_result = 0;

    while result != previous_result {
        let mut time: usize = 30;
        let mut pressure = 0;
        let mut current_valve = String::from("AA");
        let mut active_valves: HashSet<String> = HashSet::new();
        let lookup_map = valves.clone();

        // Valves with rate 0 can be omited
        valves.iter().filter(|v| v.1.rate == 0).for_each(|v| {
            active_valves.insert(v.0.to_string());
        });

        while time > 0 {
            println!("Time: {time}");
            let valve = valves
                .get(&current_valve)
                .expect("Valve should exist.{current_value} {valves:?}");

            let best_move = valve
                .neighbors
                .iter()
                .filter(|n| !active_valves.contains(&n.name) && time >= n.distance)
                .map(|n| {
                    (
                        n,
                        lookup_map
                            .get(&n.name)
                            .expect("Should exist")
                            .highest_expected_value(
                                time - n.distance,
                                active_valves.clone(),
                                &lookup_map,
                                max_distance,
                            ),
                    )
                })
                // .map(|a| {
                //     println!("{a:?}");
                //     a
                // })
                .max_by_key(|winner| winner.1)
                .map(|(n, _)| n);

            match best_move {
                Some(neighbor) => {
                    time -= neighbor.distance;
                    pressure += neighbor.rate * time;
                    current_valve = neighbor.name.clone();
                    active_valves.insert(current_valve.clone());
                    println!(
                        "Best move from {} is {} with value {}",
                        valve.name,
                        neighbor.name,
                        neighbor.rate * time
                    );
                }
                None => time = 0,
            }
        }

        previous_result = result;
        result = pressure;
        println!(" For distance {} result is {}", max_distance, result);
        max_distance += 1;
    }
    let elapsed = start_time.elapsed();
    print!("AOC day16 part1: Released pressure: {result} in {elapsed:?}")
}

#[derive(Debug, Clone)]
struct Valve {
    rate: usize,
    name: String,
    tunnels: Vec<String>,
    neighbors: Vec<Neighbor>,
}

impl Valve {
    fn detect_neighbors(&mut self, positions: &HashMap<String, Valve>) {
        let mut visited_valves: HashSet<String> = HashSet::new();
        let mut neighbors: Vec<Neighbor> = vec![];
        let mut temp: Vec<&Valve> = Vec::new();
        let mut counter = 0;

        temp.push(self);

        while !temp.is_empty() {
            let mut next_valves: Vec<&Valve> = Vec::new();
            for _ in 0..temp.len() {
                let v: &Valve = temp.pop().expect("Shouldnt happen");

                // Check if it is a new location
                if !visited_valves.contains(&v.name) {
                    neighbors.push(Neighbor {
                        name: v.name.clone(),
                        rate: v.rate,
                        distance: counter + 1,
                    });
                    visited_valves.insert(v.name.clone());
                }

                // look for unknown neighbors
                for valve in v
                    .tunnels
                    .iter()
                    .map(|tunnel| positions.get(tunnel).expect("Valve has to exist."))
                {
                    if !visited_valves.contains(&valve.name) {
                        next_valves.push(valve);
                    }
                }
            }

            // Increase distance for next pass
            counter += 1;
            temp.append(&mut next_valves);
        }

        self.neighbors = neighbors;
    }

    fn highest_expected_value(
        &self,
        remaining_time: usize,
        mut open_valves: HashSet<String>,
        lookup_map: &HashMap<String, Valve>,
        max_distance: usize,
    ) -> usize {
        if remaining_time < 1 || open_valves.len() == lookup_map.len() {
            return 0;
        }

        open_valves.insert(self.name.clone());
        let value = self.rate * remaining_time;
        self.neighbors
            .iter()
            .filter(|n| {
                !open_valves.contains(&n.name)
                    && remaining_time >= n.distance
                    && n.distance < max_distance
            })
            .map(|n| (n, lookup_map.get(&n.name).expect("Should exist")))
            .map(|(n, v)| {
                value
                    + v.highest_expected_value(
                        remaining_time - n.distance,
                        open_valves.clone(),
                        lookup_map,
                        max_distance,
                    )
            })
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
struct Neighbor {
    name: String,
    rate: usize,
    distance: usize,
}
