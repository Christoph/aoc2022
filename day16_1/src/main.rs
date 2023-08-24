use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{complete::digit1, streaming::alpha1},
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

    // println!("{valves:?}");

    let mut time = 30;
    let mut pressure = 0;
    let mut current_valve = String::from("AA");
    let mut active_valves: HashSet<String> = HashSet::new();

    while time > 0 {
        // Get best move
        let valve = valves
            .get_mut(&current_valve)
            .expect("Valve should exist.{current_value} {valves:?}");
        let best_move = valve.best_move(time, &active_valves);

        // Execute move + 1 for activating if a best move exists
        // Stop the loop if no best move exists
        match best_move {
            Some(neighbor) => {
                println!(
                    "Best move from {} is {} with value {}",
                    valve.name, neighbor.name, neighbor.value
                );
                time -= neighbor.distance;
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

    fn best_move(
        &mut self,
        remaining_time: usize,
        open_valves: &HashSet<String>,
    ) -> Option<Neighbor> {
        let winner = self
            .neighbors
            .iter_mut()
            .filter(|n| remaining_time.checked_sub(n.distance).is_some())
            .map(|n| {
                n.value(remaining_time, open_valves);
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
}

#[derive(Debug, Clone)]
struct Neighbor {
    name: String,
    rate: usize,
    distance: usize,
    value: usize,
}
