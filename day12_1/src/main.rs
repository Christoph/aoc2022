use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct SearchRay<'a> {
    current: &'a Point,
    last: &'a Point,
}

fn main() {
    let mut positions: HashMap<Point, usize> = HashMap::default();
    let mut letterMapper: HashMap<char, usize> = (b'a'..=b'z')
        .enumerate()
        .map(|(i, c)| (c as char, i + 1))
        .collect();
    letterMapper.insert('S', 0);
    letterMapper.insert('E', 27);
    let mut start_position = Point { x: 0, y: 0 };
    let mut search_rays: Vec<SearchRay> = Vec::new();

    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(col_index, character)| {
                if character == 'S' {
                    start_position = Point {
                        x: col_index,
                        y: line_index,
                    }
                };
                positions.insert(
                    Point {
                        x: col_index,
                        y: line_index,
                    },
                    *letterMapper.get(&character).unwrap_or(&28),
                );
            })
        });

    println!("Start position: {start_position:?}");
    search_rays.push(SearchRay {
        current: &start_position,
        last: &start_position,
    });

    while search_rays.len() > 0 {
        search_rays.iter_mut().map(|ray| {
            let c_e = positions.get(&ray.current).unwrap_or(&28);
            let n_e = positions.get(&ray.current.clone().x -= 1).unwrap_or(&28);
            let e_e = positions.get(&ray.current).unwrap_or(&28);
            let s_e = positions.get(&ray.current).unwrap_or(&28);
            let w_e = positions.get(&ray.current).unwrap_or(&28);
            // Check north
            if positions.get(&ray.current) {}
        });
    }
}
