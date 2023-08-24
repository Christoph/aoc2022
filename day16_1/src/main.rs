use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{complete::digit1, streaming::alpha1},
    IResult,
};

impl Neighbor {
    fn value(&self, remaining_time: usize) -> usize {
        (remaining_time - self.distance) * self.rate
    }
}

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

    // println!("{valves:?}");

    let mut time: usize = 30;
    let mut pressure = 0;
    let mut current_valve = String::from("AA");
    let mut active_valves: HashSet<String> = HashSet::new();

    // Valves with rate 0 can be omited
    valves.iter().filter(|v| v.1.rate == 0).for_each(|v| {
        active_valves.insert(v.0.to_string());
    });

    while time > 0 {
        println!("Time: {time}{:?}", active_valves);
        // Get best move
        let valve = valves
            .get_mut(&current_valve)
            .expect("Valve should exist.{current_value} {valves:?}");

        let best_move = valve
            .neighbors
            .iter()
            .filter(|n| !active_valves.contains(&n.name) && time.checked_sub(n.distance).is_some())
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
                println!("{a:?}");
                a
            })
            .max_by_key(|winner| winner.1)
            .map(|(n, _)| n);

        // Try 1 - not working
        // let best_move = valve.best_move(time, &active_valves, &lookup_map);

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
                        value: 0,
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
        let valve_values: Vec<(&Neighbor, &Valve)> = self
            .neighbors
            .iter()
            .filter(|n| {
                !open_valves.contains(&n.name) && remaining_time.checked_sub(n.distance).is_some()
            })
            .map(|n| (n, lookup_map.get(&n.name).expect("Should exist")))
            .collect();

        // if !valve_values.is_empty() {
        //     valve_values
        //         .iter()
        //         .filter(|(n, _)| remaining_time.checked_sub(n.distance).is_some())
        //         .map(|(n, v)| {
        //             value
        //                 + v.highest_expected_value(
        //                     remaining_time - n.distance,
        //                     open_valves.clone(),
        //                     lookup_map,
        //                 )
        //         })
        //         .sum::<usize>()
        //         / valve_values.len()
        // } else {
        //     0
        // }
        value
    }

    fn best_move(
        &mut self,
        remaining_time: usize,
        open_valves: &HashSet<String>,
        lookup_map: &HashMap<String, Valve>,
    ) -> Option<Neighbor> {
        let neighborhood_average_value =
            self.neighbor_average_value(remaining_time, lookup_map, open_valves);
        // println!("average value: {}", neighborhood_average_value);
        // println!("{self:?}");
        let winner = self
            .neighbors
            .iter_mut()
            .filter(|n| {
                !open_valves.contains(&n.name) && remaining_time.checked_sub(n.distance).is_some()
            })
            .map(|n| {
                n.value += n.value(remaining_time) + neighborhood_average_value;
                n
            })
            .max_by_key(|n| n.value)
            .expect("No neighbor");

        println!("winner: {winner:?}");
        if winner.value == 0 {
            None
        } else {
            Some(winner.clone())
        }
    }

    fn neighbor_average_value(
        &mut self,
        remaining_time: usize,
        lookup_map: &HashMap<String, Valve>,
        open_valves: &HashSet<String>,
    ) -> usize {
        let values: Vec<usize> = self
            .neighbors
            .clone()
            .iter()
            .filter(|n| n.distance < 2)
            .map(|n| {
                let v = lookup_map.get(&n.name).expect("Should exist");
                v.neighbors
                    .iter()
                    .filter(|n| {
                        !open_valves.contains(&n.name)
                            && remaining_time.checked_sub(n.distance).is_some()
                    })
                    .map(|other| other.value(remaining_time))
                    .sum::<usize>()
                    / v.neighbors.len()
            })
            .collect();

        let length = values.len();
        if length > 0 {
            values.iter().sum::<usize>() / length
        } else {
            0
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
