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

    println!("I start from AA");
    while time > 0 {
        println!("Time: {time}");
        let best_moves_elefant: Vec<(&Neighbor, usize)>;
        let mut best_move_elefant: Option<&(&Neighbor, usize)> = None;
        let mut best_move_elefant_alternative: Option<&(&Neighbor, usize)> = None;
        let best_moves_me: Vec<(&Neighbor, usize)>;
        let mut best_move_me: Option<&(&Neighbor, usize)> = None;
        let mut best_move_me_alternative: Option<&(&Neighbor, usize)> = None;

        if working_elefant < 1 {
            let valve = valves
                .get(&current_elefant)
                .expect("Valve should exist.{current_value} {valves:?}");

            best_moves_elefant = valve
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
                                time - n.distance - 1,
                                active_valves.clone(),
                                &lookup_map,
                            ),
                    )
                })
                .sorted_by_key(|entry| entry.1)
                .rev()
                .collect();

            // println!("Best moves elefant: {:?}", best_moves_elefant);
            best_move_elefant = best_moves_elefant.get(0);
            best_move_elefant_alternative = best_moves_elefant.get(1);
        } else {
            working_elefant -= 1;
        }

        if working_me < 1 {
            let valve = valves
                .get(&current_me)
                .expect("Valve should exist.{current_value} {valves:?}");

            best_moves_me = valve
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
                .sorted_by_key(|entry| entry.1)
                .rev()
                .collect();

            println!("Best moves me: {:?}", best_moves_me);
            best_move_me = best_moves_me.get(0);
            best_move_me_alternative = best_moves_me.get(1);
        } else {
            working_me -= 1;
        }

        // Coordinate next steps
        // Same target -> Redirect the lower value
        match (best_move_me, best_move_elefant) {
            (None, None) => (),
            (None, Some(ele)) => {
                working_elefant = ele.0.distance - 1;
                current_elefant = ele.0.name.clone();
                active_valves.insert(current_elefant.clone());
                pressure += ele.0.rate * (time - working_elefant - 1);
                println!("Elefant moves to {}", current_elefant);
            }
            (Some(me), None) => {
                working_me = me.0.distance - 1;
                current_me = me.0.name.clone();
                active_valves.insert(current_me.clone());
                pressure += me.0.rate * (time - working_me - 1);
                println!("I move to {}", current_me);
            }
            (Some(me), Some(ele)) => {
                if me.0.name == ele.0.name {
                    if me.1 >= ele.1 {
                        working_me = me.0.distance - 1;
                        current_me = me.0.name.clone();
                        active_valves.insert(current_me.clone());
                        pressure += me.0.rate * (time - working_me - 1);
                        println!("I move to {}", current_me);

                        let ele = best_move_elefant_alternative.unwrap();
                        working_elefant = ele.0.distance - 1;
                        current_elefant = ele.0.name.clone();
                        active_valves.insert(current_elefant.clone());
                        pressure += ele.0.rate * (time - working_elefant - 1);
                        println!("Elefant moves to {}", current_elefant);
                    } else {
                        let me = best_move_me_alternative.unwrap();
                        working_me = me.0.distance - 1;
                        current_me = me.0.name.clone();
                        active_valves.insert(current_me.clone());
                        pressure += me.0.rate * (time - working_me - 1);
                        println!("I move to {}", current_me);

                        working_elefant = ele.0.distance - 1;
                        current_elefant = ele.0.name.clone();
                        active_valves.insert(current_elefant.clone());
                        pressure += ele.0.rate * (time - working_elefant - 1);
                        println!("Elefant moves to {}", current_elefant);
                    }
                } else {
                    working_me = me.0.distance - 1;
                    current_me = me.0.name.clone();
                    active_valves.insert(current_me.clone());
                    pressure += me.0.rate * (time - working_me - 1);
                    println!("I move to {}", current_me);

                    working_elefant = ele.0.distance - 1;
                    current_elefant = ele.0.name.clone();
                    active_valves.insert(current_elefant.clone());
                    pressure += ele.0.rate * (time - working_elefant - 1);
                    println!("Elefant moves to {}", current_elefant);
                }
            }
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
    ) -> usize {
        if remaining_time < 1 || open_valves.len() == lookup_map.len() {
            return 0;
        }

        open_valves.insert(self.name.clone());
        let value = self.rate * remaining_time;
        self.neighbors
            .iter()
            .filter(|n| {
                !open_valves.contains(&n.name) && remaining_time >= n.distance // && n.distance < 6
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
