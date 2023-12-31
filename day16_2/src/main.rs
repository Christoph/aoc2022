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

    let lookup_map = valves.clone();

    let mut time: usize = 26;
    let mut pressure = 0;
    let mut current_me = String::from("AA");
    let mut working_me = 0;
    let mut current_elefant = String::from("AA");
    let mut working_elefant = 0;
    let mut active_valves: HashSet<String> = HashSet::new();

    // Valves with rate 0 can be omited
    valves.iter().filter(|v| v.1.rate == 0).for_each(|v| {
        active_valves.insert(v.0.to_string());
    });

    valves
        .iter_mut()
        .for_each(|v| v.1.neighbors.retain(|v| v.rate > 0));

    println!("I start from AA");
    while time > 0 {
        println!("Time: {time}");
        let mut best_move_elefant: Option<&Neighbor> = None;
        let mut best_move_me: Option<&Neighbor> = None;

        if working_elefant < 1 {
            let valve = valves
                .get(&current_elefant)
                .expect("Valve should exist.{current_value} {valves:?}");

            best_move_elefant = valve
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
                                time - (n.distance),
                                active_valves.clone(),
                                &lookup_map,
                            ),
                    )
                })
                .map(|a| {
                    println!("Best moves for elefant {a:?}");
                    a
                })
                .max_by_key(|winner| winner.1)
                .map(|(n, _)| n);
        } else {
            working_elefant -= 1;
        }

        if let Some(ele) = best_move_elefant {
            working_elefant = ele.distance;
            current_elefant = ele.name.clone();
            active_valves.insert(current_elefant.clone());
            pressure += ele.rate * (time - working_elefant);
            println!("Elefant moves to {}", current_elefant);
        }

        if working_me < 1 {
            let valve = valves
                .get(&current_me)
                .expect("Valve should exist.{current_value} {valves:?}");

            best_move_me = valve
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
                            ),
                    )
                })
                .map(|a| {
                    println!(" Best moves for me {a:?}");
                    a
                })
                .max_by_key(|winner| winner.1)
                .map(|(n, _)| n);
        } else {
            working_me -= 1;
        }

        if let Some(me) = best_move_me {
            working_me = me.distance;
            current_me = me.name.clone();
            active_valves.insert(current_me.clone());
            pressure += me.rate * (time - working_me);
            println!("I move to {}", current_me);
        }

        time -= 1;
    }
    let elapsed = start_time.elapsed();
    print!("Released pressure: {pressure} in {elapsed:?}")
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

                // Check if it is a new location with a positive rate
                if !visited_valves.contains(&v.name) {
                    neighbors.push(Neighbor {
                        name: v.name.clone(),
                        rate: v.rate,
                        distance: counter,
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
    ) -> usize {
        if remaining_time < 1 || open_valves.len() == lookup_map.len() {
            return 0;
        }

        open_valves.insert(self.name.clone());
        let value = self.rate * remaining_time;
        self.neighbors
            .iter()
            .filter(|n| {
                !open_valves.contains(&n.name) && remaining_time >= n.distance // && n.distance < 7
            })
            .map(|n| (n, lookup_map.get(&n.name).expect("Should exist")))
            .map(|(n, v)| {
                value
                    + v.highest_expected_value(
                        remaining_time - n.distance,
                        open_valves.clone(),
                        lookup_map,
                    )
            })
            // .max()
            // .unwrap_or(0)
            .sum::<usize>()
            / self.neighbors.len()
    }
}

#[derive(Debug, Clone)]
struct Neighbor {
    name: String,
    rate: usize,
    distance: usize,
}
