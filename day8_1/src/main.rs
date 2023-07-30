fn main() {
    let mut visible_indices: Vec<String> = vec![];

    let data: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() + 1) // Offeset by one to allow easier work later
                .collect::<Vec<u32>>()
        })
        .collect();

    // From top and bottom
    let row_size = data.first().unwrap().len();

    for x in 0..row_size {
        let col_size = data.len();
        let mut col_max = 0;

        // From top
        for (y, row) in data.iter().enumerate() {
            let element = row.get(x).unwrap();

            if *element > col_max {
                col_max = *element;
                let index = format!("x{x}y{y}");
                if !visible_indices.contains(&index) {
                    visible_indices.push(index);
                }
            }
        }
        col_max = 0;

        // From bottom
        for (y, row) in data.iter().rev().enumerate() {
            let element = row.get(x).unwrap();

            if *element > col_max {
                col_max = *element;
                let normalized_index = col_size - y - 1;
                let index = format!("x{x}y{normalized_index}");
                if !visible_indices.contains(&index) {
                    visible_indices.push(index);
                }
            }
        }
    }

    // From Left and right
    for (y, row) in data.iter().enumerate() {
        let row_size = row.len();
        let mut row_max = 0;

        // from left
        for (x, element) in row.iter().enumerate() {
            if *element > row_max {
                row_max = *element;
                let index = format!("x{x}y{y}");
                if !visible_indices.contains(&index) {
                    visible_indices.push(index);
                }
            }
        }
        row_max = 0;
        // from right
        for (x, element) in row.iter().rev().enumerate() {
            if *element > row_max {
                row_max = *element;
                let normalized_index = row_size - x - 1;
                let index = format!("x{normalized_index}y{y}");
                if !visible_indices.contains(&index) {
                    visible_indices.push(index);
                }
            }
        }
    }

    println!("{visible_indices:?}");
    let n_visible_trees = visible_indices.len();

    println!("{n_visible_trees:?}");
}
