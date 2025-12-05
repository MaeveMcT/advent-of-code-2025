use rust::{get_input_file_path, read_input};

#[derive(Clone)]
struct Grid {
    width: u32,
    height: u32,
    grid: Vec<Vec<char>>,
}

type Pos = (usize, usize);

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            return line.chars().collect();
        })
        .collect();

    let mut grid = Grid {
        width: grid[0].len() as u32, // Assumes all rows are same width
        height: grid.len() as u32,
        grid,
    };

    print_grid(&grid);
    let adj = adjacent_positions_in_grid(&grid, 1, 1);

    let mut accessible_rows_of_paper = 0;
    loop {
        let mut accessible_paper_positions = vec![];
        for (i, row) in grid.grid.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == '@' {
                    if forklift_can_access(&grid, i, j) {
                        accessible_rows_of_paper += 1;
                        accessible_paper_positions.push((i, j));
                    }
                }
            }
        }

        let new_grid = remove_accessible_paper_from(&grid, &accessible_paper_positions);
        if new_grid.grid == grid.grid {
            break;
        }
        grid = new_grid;
    }

    for pos in &adj {
        println!("{}", grid.grid[pos.0][pos.1]);
    }

    println!("Accessible rows of paper: {}", accessible_rows_of_paper);
}

fn remove_accessible_paper_from(grid: &Grid, accessible_positions: &Vec<(usize, usize)>) -> Grid {
    let mut new_grid = grid.clone();

    for (x, y) in accessible_positions {
        new_grid.grid[*x][*y] = 'x';
    }

    return new_grid;
}

fn forklift_can_access(grid: &Grid, x: usize, y: usize) -> bool {
    let row_of_paper = grid.grid[x][y];
    assert_eq!(
        row_of_paper, '@',
        "Didn't find roll of paper at {},{}",
        x, y
    );
    let adjacents = adjacent_positions_in_grid(grid, x, y);
    let adjacent_paper_roll_count = adjacents
        .iter()
        .filter(|pos| grid.grid[pos.0][pos.1] == '@')
        .count();

    if adjacent_paper_roll_count < 4 {
        return true;
    }
    return false;
}

fn adjacent_positions_in_grid(grid: &Grid, x: usize, y: usize) -> Vec<Pos> {
    let mut adjacent_positions = vec![];

    let x = x as i32;
    let y = y as i32;
    for i in -1..=1 {
        let test_x = x + i;
        if test_x < 0 || test_x >= grid.width as i32 {
            continue;
        }
        for j in -1..=1 {
            let test_y = y + j;

            if j == 0 && i == 0 {
                // Don't count our own point!
                continue;
            }
            if test_y < 0 || test_y >= grid.height as i32 {
                continue;
            }
            adjacent_positions.push((test_x as usize, test_y as usize));
        }
    }

    return adjacent_positions;
}

fn print_grid(grid: &Grid) {
    for row in &grid.grid {
        for &col in row {
            print!("{}", col);
        }
        println!("");
    }
}
