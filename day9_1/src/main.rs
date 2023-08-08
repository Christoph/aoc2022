use std::collections::HashSet;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Clone)]
enum Directions {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Movement {
    direction: Directions,
    distance: u32,
}

impl Movement {
    fn new(direction: &str, distance: u32) -> Self {
        Movement {
            direction: match direction {
                "R" => Directions::Right,
                "L" => Directions::Left,
                "U" => Directions::Up,
                "D" => Directions::Down,
                _ => panic!("Unknown Direction!"),
            },
            distance,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn follow(&self, other: Position) -> Position {
        let x_distance = other.x - self.x;
        let x_distance_abs = x_distance.unsigned_abs();
        let x_distance_norm;
        let y_distance = other.y - self.y;
        let y_distance_abs = y_distance.unsigned_abs();
        let y_distance_norm;

        println!("{other:?}/{self:?}");
        println!("{x_distance}/{y_distance}");

        if x_distance_abs + y_distance_abs > 3 {
            panic!("Distance is too big - check loop")
        }

        // Normalize distances
        if x_distance >= 1 {
            x_distance_norm = x_distance - 1;
        } else if x_distance <= -1 {
            x_distance_norm = x_distance + 1;
        } else {
            x_distance_norm = x_distance;
        }

        if y_distance >= 1 {
            y_distance_norm = y_distance - 1;
        } else if y_distance <= -1 {
            y_distance_norm = y_distance + 1;
        } else {
            y_distance_norm = y_distance;
        }

        if x_distance_abs == 2 && y_distance_abs == 0 {
            // x movement
            Position {
                x: self.x + x_distance_norm,
                y: self.y,
            }
        } else if x_distance_abs == 0 && y_distance_abs == 2 {
            // y movement
            Position {
                x: self.x,
                y: self.y + y_distance_norm,
            }
        } else if x_distance_abs + y_distance_abs == 3 {
            let y_distance_diag;
            let x_distance_diag;

            if y_distance.is_negative() {
                y_distance_diag = -1;
            } else {
                y_distance_diag = 1;
            }

            if x_distance.is_negative() {
                x_distance_diag = -1;
            } else {
                x_distance_diag = 1;
            }

            // dialognal movement
            Position {
                x: self.x + x_distance_diag,
                y: self.y + y_distance_diag,
            }
        } else {
            // no movement
            Position {
                x: self.x,
                y: self.y,
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Rope {
    elements: Vec<Position>,
}

impl Rope {
    fn resolve_movement(&mut self, mov: Movement, positions: &mut HashSet<Position>) {
        let mut updated = self.elements.clone();
        for _ in 0..mov.distance {
            for ((p_index, _), (n_index, _)) in self.elements.iter().enumerate().tuple_windows() {
                if p_index == 0 {
                    // Head
                    match mov.direction {
                        Directions::Right => updated.get_mut(p_index).unwrap().x += 1,
                        Directions::Left => updated.get_mut(p_index).unwrap().x -= 1,
                        Directions::Up => updated.get_mut(p_index).unwrap().y += 1,
                        Directions::Down => updated.get_mut(p_index).unwrap().y -= 1,
                    }
                }
                // Tail
                let new_position = updated[n_index].follow(*updated.get(p_index).unwrap());
                let b = *updated.get(p_index).unwrap();
                println!("{b:?} -> {new_position:?}");
                if n_index == self.elements.len() - 1 {
                    positions.insert(new_position);
                }

                updated[n_index] = new_position;
            }
        }
        self.elements = updated;
    }
}

fn parse_line(line: &str) -> IResult<&str, Movement> {
    map(
        separated_pair(alpha1, tag(" "), digit1),
        |(direction, distance)| Movement::new(direction, distance.parse().unwrap()),
    )(line)
}

fn main() {
    let data: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();
    let mut positions = HashSet::new();

    let mut rope = Rope {
        elements: vec![
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
        ],
    };

    for movement in data {
        rope.resolve_movement(movement, &mut positions);
        println!("{rope:?}")
    }

    let result = positions.len();

    println!("{rope:?} => {result}");
}
