use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{all_consuming, map};
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn follow(&self, other: Position) -> Position {
        let x_distance = self.x - other.x;
        let y_distance = self.y - other.y;

        if x_distance.abs() > 2 || y_distance.abs() > 2 {
            panic!("DIstance is too big")
        }

        if x_distance.unsigned_abs() > 1 && y_distance.unsigned_abs() <= 1 {
            // x movement
            Position {
                x: other.x + x_distance - 1,
                y: self.y,
            }
        } else if x_distance.unsigned_abs() <= 1 && y_distance.unsigned_abs() > 1 {
            // y movement position
            Position {
                x: self.x,
                y: other.y + y_distance - 1,
            }
        } else if x_distance > 1 && y_distance > 1 {
            // dialognal movement
            Position {
                x: other.x + x_distance - 1,
                y: other.y + y_distance - 1,
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
    fn resolve_movement(&mut self, mov: Movement) {
        println!("{self:?}");
        let mut updated = self.elements.clone();
        for _ in 0..mov.distance {
            for ((p_index, prev), (_, next)) in self.elements.iter().enumerate().tuple_windows() {
                // Head
                if p_index == 0 {
                    match mov.direction {
                        Directions::Right => updated.get_mut(p_index).unwrap().x += 1,
                        Directions::Left => updated.get_mut(p_index).unwrap().x -= 1,
                        Directions::Up => updated.get_mut(p_index).unwrap().y += 1,
                        Directions::Down => updated.get_mut(p_index).unwrap().y -= 1,
                    }
                }

                // Tail
            }
        }
        self.elements = updated;
        println!("{self:?}")
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
        .map(|line| all_consuming(parse_line)(line).unwrap().1)
        .collect();

    let mut rope = Rope {
        elements: vec![Position { x: 0, y: 0 }, Position { x: 0, y: 0 }],
    };

    for movement in data {
        rope.resolve_movement(movement);
    }

    println!("{rope:?}");
}
