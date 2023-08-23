use std::{
    collections::{HashMap, HashSet},
    ops::IndexMut,
};

#[derive(Debug)]
struct SearchRay {
    current: (usize, usize),
    steps: usize,
}

fn main() {
    let mut positions: HashMap<(usize, usize), usize> = HashMap::default();
    let mut touched_positions: HashSet<(usize, usize)> = HashSet::default();
    let mut letterMapper: HashMap<char, usize> = (b'a'..=b'z')
        .enumerate()
        .map(|(i, c)| (c as char, i + 1))
        .collect();
    letterMapper.insert('S', 0);
    letterMapper.insert('E', 27);
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    let mut search_rays: Vec<SearchRay> = Vec::new();
    let max_elevation = 1;

    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(col_index, character)| {
                if character == 'S' {
                    start_position = (col_index, line_index);
                };
                if character == 'E' {
                    end_position = (col_index, line_index);
                };
                positions.insert(
                    (col_index, line_index),
                    *letterMapper.get(&character).unwrap_or(&usize::MAX),
                );
            })
        });

    println!("Start position: {start_position:?}");
    search_rays.push(SearchRay {
        current: start_position,
        steps: 0,
    });
    touched_positions.insert(start_position);

    while !search_rays.is_empty() {
        let mut temp: Vec<SearchRay> = Vec::new();
        for _ in 0..search_rays.len() {
            let ray = search_rays.pop().unwrap();
            let n_ray = (ray.current.0, ray.current.1.saturating_sub(1));
            let e_ray = (ray.current.0 + 1, ray.current.1);
            let s_ray = (ray.current.0, ray.current.1 + 1);
            let w_ray = (ray.current.0.saturating_sub(1), ray.current.1);

            let c_e = positions.get(&ray.current).unwrap_or(&usize::MAX);

            // Check north
            if !touched_positions.contains(&n_ray) {
                let n_e = positions.get(&n_ray).unwrap_or(&usize::MAX);
                if n_e.saturating_sub(*c_e) <= max_elevation {
                    temp.push(SearchRay {
                        current: n_ray,
                        steps: ray.steps + 1,
                    });
                    touched_positions.insert(n_ray);
                }
            }
            // Check east
            if !touched_positions.contains(&e_ray) {
                let e_e = positions.get(&e_ray).unwrap_or(&usize::MAX);
                if e_e.saturating_sub(*c_e) <= max_elevation {
                    temp.push(SearchRay {
                        current: e_ray,
                        steps: ray.steps + 1,
                    });
                    touched_positions.insert(e_ray);
                }
            }
            // Check South
            if !touched_positions.contains(&s_ray) {
                let s_e = positions.get(&s_ray).unwrap_or(&usize::MAX);
                if s_e.saturating_sub(*c_e) <= max_elevation {
                    temp.push(SearchRay {
                        current: s_ray,
                        steps: ray.steps + 1,
                    });
                    touched_positions.insert(s_ray);
                }
            }
            // Check West
            if !touched_positions.contains(&w_ray) {
                let w_e = positions.get(&w_ray).unwrap_or(&usize::MAX);
                if w_e.saturating_sub(*c_e) <= max_elevation {
                    temp.push(SearchRay {
                        current: w_ray,
                        steps: ray.steps + 1,
                    });
                    touched_positions.insert(w_ray);
                }
            }

            // Reached dead end
            if ray.current == end_position {
                println!("Fasts path has {} steps.", ray.steps);
                break;
            }
        }

        // println!("{temp:?}{touched_positions:?}");
        search_rays.append(&mut temp)
    }
}
