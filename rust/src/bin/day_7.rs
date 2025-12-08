use rust::{get_input_file_path, read_input};

#[derive(Clone)]
struct Grid {
    width: u32,
    height: u32,
    grid: Vec<Vec<GridItem>>,
}

#[derive(Clone, PartialEq, Eq)]
enum GridItem {
    Unoccupied,
    Beam(u64),
    Splitter,
}

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            return line.chars().collect();
        })
        .collect();

    let grid: Vec<Vec<_>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|ch| match ch {
                    '|' | 'S' => GridItem::Beam(1),
                    '^' => GridItem::Splitter,
                    _ => GridItem::Unoccupied,
                })
                .collect()
        })
        .collect();

    let mut grid = Grid {
        width: grid[0].len() as u32, // Assumes all rows are same width
        height: grid.len() as u32,
        grid,
    };
    print_grid(&grid);
    for row_idx in 1..grid.height as usize {
        grid = propogate_beams(grid);
        grid = split_beams(grid, row_idx);
        println!("===");
        print_grid(&grid);
    }

    let total_paths_taken = grid
        .grid
        .last()
        .unwrap()
        .iter()
        .fold(0, |acc, item| match item {
            GridItem::Unoccupied => acc + 0,
            GridItem::Beam(collisions) => acc + collisions,
            GridItem::Splitter => acc + 0,
        });
    println!("Total paths taken: {}", total_paths_taken);
}

fn print_grid(grid: &Grid) {
    for row in &grid.grid {
        for col in row {
            let s = match col {
                GridItem::Unoccupied => ".".to_string(),
                GridItem::Beam(colisions) => format!("{}", colisions),
                GridItem::Splitter => "^".to_string(),
            };
            print!("{}", s);
        }
        println!("");
    }
}
fn propogate_beams(mut grid: Grid) -> Grid {
    let mut row_with_beams_propogated = vec![];
    let mut row_idx = 1;
    while row_idx < grid.height as usize {
        let row = &grid.grid[row_idx];
        let previous_row = &grid.grid[row_idx - 1];
        // iterate through rows until no beams are found
        if row.iter().any(|item| matches!(item, GridItem::Beam(_))) {
            row_idx += 1;
            continue;
        } else {
            // Get beam positions from row - 1
            let mut beams_in_previous_row = vec![];
            for (i, item) in previous_row.iter().enumerate() {
                if let GridItem::Beam(collisions) = item {
                    beams_in_previous_row.push((i, collisions));
                }
            }
            let mut new_row = row.clone();
            for (beam_pos, collisions) in beams_in_previous_row {
                if !matches!(new_row[beam_pos], GridItem::Splitter) {
                    new_row[beam_pos] = GridItem::Beam(*collisions);
                }
            }
            row_with_beams_propogated = new_row;
            break;

            // mutate current row so that row[beam_position] = '|' unless row[beam_position] is a
            // splitter
        }
    }
    if !row_with_beams_propogated.is_empty() {
        grid.grid[row_idx] = row_with_beams_propogated;
    }
    return grid;
}

fn find_splits_for_row(row: &Vec<GridItem>) -> Vec<usize> {
    let mut split_positions = vec![];
    for (i, item) in row.iter().enumerate() {
        if let GridItem::Splitter = item {
            split_positions.push(i);
        }
    }
    return split_positions;
}
fn split_beams(mut grid: Grid, row_idx: usize) -> Grid {
    let mut row = grid.grid[row_idx].clone();
    let previous_row = &grid.grid[row_idx - 1];
    let splits_in_row = find_splits_for_row(&row);

    for split in splits_in_row {
        if let GridItem::Beam(collisions) = previous_row[split] {
            // Split it!
            if split > 0 {
                let left_item = &mut row[split - 1];
                *left_item = match left_item {
                    GridItem::Unoccupied => GridItem::Beam(collisions),
                    GridItem::Beam(prev_collisions) => {
                        GridItem::Beam(collisions + *prev_collisions)
                    }
                    // There aren't any adjacent splitters in the puzzle input AFAIK
                    GridItem::Splitter => unimplemented!(),
                };
            }
            if split < row.len() {
                let right_item = &mut row[split + 1];
                *right_item = match right_item {
                    GridItem::Unoccupied => GridItem::Beam(collisions),
                    GridItem::Beam(prev_collisions) => {
                        GridItem::Beam(collisions + *prev_collisions)
                    }
                    // There aren't any adjacent splitters in the puzzle input AFAIK
                    GridItem::Splitter => unimplemented!(),
                };
            }
        }
    }

    grid.grid[row_idx] = row;
    return grid;
}
