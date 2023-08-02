use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct VisIndex {
    l_vis: u32,
    r_vis: u32,
    u_vis: u32,
    d_vis: u32,
}

impl VisIndex {
    fn scenic_score(&self) -> u32 {
        self.d_vis * self.u_vis * self.l_vis * self.r_vis
    }
    fn sum(&self) -> u32 {
        self.d_vis + self.u_vis + self.l_vis + self.r_vis
    }
}

fn main() {
    let mut visible_indices: HashMap<String, VisIndex> = HashMap::default();

    let data: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap()) // Offeset by one to allow easier work later
                .collect::<Vec<u32>>()
        })
        .collect();

    // From top and bottom
    let row_size = data.first().unwrap().len();
    let col_size = data.len();

    // Init hashmap
    for x in 0..row_size {
        for y in 0..col_size {
            let index = format!("x{x}y{y}");
            visible_indices.insert(
                index,
                VisIndex {
                    l_vis: 0,
                    r_vis: 0,
                    u_vis: 0,
                    d_vis: 0,
                },
            );
        }
    }

    for x in 0..row_size {
        let col_size = data.len();
        let mut col_max = *data.first().unwrap().get(x).unwrap();
        let mut max_index = 0;
        let mut outer_index = 0;

        // From up
        for (y, row) in data.iter().enumerate() {
            let element = row.get(x).unwrap();

            match element.cmp(&col_max) {
                std::cmp::Ordering::Less => {
                    col_max = *element;
                    outer_index = max_index;
                    max_index = y;
                }
                std::cmp::Ordering::Equal => {
                    let index = format!("x{x}y{y}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.u_vis = u32::try_from(y - max_index).unwrap();
                    max_index = y;
                }
                std::cmp::Ordering::Greater => {
                    col_max = *element;
                    let index = format!("x{x}y{y}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.u_vis = u32::try_from(y - outer_index).unwrap();
                    max_index = y;
                }
            }
        }
        col_max = *data.last().unwrap().get(x).unwrap();
        max_index = 0;
        outer_index = 0;

        // From down
        for (y, row) in data.iter().rev().enumerate() {
            let element = row.get(x).unwrap();

            match element.cmp(&col_max) {
                std::cmp::Ordering::Less => {
                    col_max = *element;
                    outer_index = max_index;
                    max_index = y;
                }
                std::cmp::Ordering::Equal => {
                    let normalized_index = col_size - y - 1;
                    let index = format!("x{x}y{normalized_index}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.d_vis = u32::try_from(y - max_index).unwrap();
                    max_index = y;
                }
                std::cmp::Ordering::Greater => {
                    col_max = *element;
                    let normalized_index = col_size - y - 1;
                    let index = format!("x{x}y{normalized_index}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.d_vis = u32::try_from(y - outer_index).unwrap();
                    max_index = y;
                }
            }
        }
    }

    // From Left and right
    for (y, row) in data.iter().enumerate() {
        let row_size = row.len();
        let mut row_max = *row.first().unwrap();
        let mut max_index = 0;
        let mut outer_index = 0;

        // from left
        for (x, element) in row.iter().enumerate() {
            match element.cmp(&row_max) {
                std::cmp::Ordering::Less => {
                    row_max = *element;
                    outer_index = max_index;
                    max_index = x;
                }
                std::cmp::Ordering::Equal => {
                    let index = format!("x{x}y{y}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.l_vis = u32::try_from(x - max_index).unwrap();
                    max_index = x;
                }
                std::cmp::Ordering::Greater => {
                    row_max = *element;
                    let index = format!("x{x}y{y}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.l_vis = u32::try_from(x - outer_index).unwrap();
                    max_index = x;
                }
            }
        }
        row_max = *row.last().unwrap();
        max_index = 0;
        outer_index = 0;

        // from right
        for (x, element) in row.iter().rev().enumerate() {
            match element.cmp(&row_max) {
                std::cmp::Ordering::Less => {
                    row_max = *element;
                    outer_index = max_index;
                    max_index = x;
                }
                std::cmp::Ordering::Equal => {
                    let normalized_index = row_size - x - 1;
                    let index = format!("x{normalized_index}y{y}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.r_vis = u32::try_from(x - max_index).unwrap();
                    max_index = x;
                }
                std::cmp::Ordering::Greater => {
                    row_max = *element;
                    let normalized_index = row_size - x - 1;
                    let index = format!("x{normalized_index}y{y}");
                    let item = visible_indices.get_mut(&index).unwrap();
                    item.r_vis = u32::try_from(x - outer_index).unwrap();
                    max_index = x;
                }
            }
        }
    }

    // println!("{visible_indices:?}");
    let n_visible_trees = visible_indices
        .iter()
        .map(|(k, v)| (k, v.scenic_score()))
        .filter(|(k, v)| *v > 0)
        .max_by_key(|r| r.1)
        .unwrap();

    println!("{n_visible_trees:?}");
}
