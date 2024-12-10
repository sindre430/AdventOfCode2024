use std::sync::{mpsc, Arc};
use std::thread;

pub fn run_task_1(data: String) {
    let grid = parse_input(data);
    let zero_coords = find_coordinates(&grid, 0);

    let grid = Arc::new(grid);

    let (tx, rx) = mpsc::channel();
    for coords in zero_coords.into_iter() {
        let tx = tx.clone();
        let grid = Arc::clone(&grid);
        thread::spawn(move || {
            let mut next_number = 1;
            let mut cur_coords = vec![coords];
            while next_number < 10 {
                let coords_to_process = cur_coords.clone();
                cur_coords.clear();
                for coords in coords_to_process.iter() {
                    let neighbors = get_neighbors(&grid, coords.0, coords.1);
                    for (neighbor_coords, neighbor_value) in neighbors {
                        if neighbor_value == next_number {
                            cur_coords.push(neighbor_coords);
                        }
                    }
                }

                next_number += 1;
            }

            // Only unique coordinates are needed.
            cur_coords = cur_coords.into_iter().fold(Vec::new(), |mut acc, coords| {
                if !acc.contains(&coords) {
                    acc.push(coords);
                }
                acc
            });

            tx.send(cur_coords.len()).unwrap();
        });
    }

    drop(tx);

    let mut sum = 0;
    for eq_sum in rx {
        sum += eq_sum;
    }

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {
    let grid = parse_input(data);
    let zero_coords = find_coordinates(&grid, 0);

    let grid = Arc::new(grid);

    let (tx, rx) = mpsc::channel();
    for coords in zero_coords.into_iter() {
        let tx = tx.clone();
        let grid = Arc::clone(&grid);
        thread::spawn(move || {
            let mut next_number = 1;
            let mut cur_coords = vec![coords];
            while next_number < 10 {
                let coords_to_process = cur_coords.clone();
                cur_coords.clear();
                for coords in coords_to_process.iter() {
                    let neighbors = get_neighbors(&grid, coords.0, coords.1);
                    for (neighbor_coords, neighbor_value) in neighbors {
                        if neighbor_value == next_number {
                            cur_coords.push(neighbor_coords);
                        }
                    }
                }

                next_number += 1;
            }

            tx.send(cur_coords.len()).unwrap();
        });
    }

    drop(tx);

    let mut sum = 0;
    for eq_sum in rx {
        sum += eq_sum;
    }

    println!("Sum: {}", sum);
}

fn find_coordinates(grid: &Vec<Vec<u32>>, target: u32) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if value == target {
                coordinates.push((row_index, col_index));
            }
        }
    }

    coordinates
}

fn get_neighbors(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> Vec<((usize, usize), u32)> {
    let mut neighbors = Vec::new();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for &(dr, dc) in &directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[new_row as usize].len() as isize
        {
            let new_row_usize = new_row as usize;
            let new_col_usize = new_col as usize;
            neighbors.push((
                (new_row_usize, new_col_usize),
                grid[new_row_usize][new_col_usize],
            ));
        }
    }

    neighbors
}

fn parse_input(data: String) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}
