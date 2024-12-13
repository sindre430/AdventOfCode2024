use std::os::windows::process;
use std::sync::{mpsc, Arc};
use std::thread;

fn process_stone(stone: i64) -> Vec<i64> {
    let mut res = Vec::new();
    if stone == 0{
        res.push(1);
        return res;
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let mid = stone_str.len() / 2;
        let parts = stone_str.split_at(mid);
        res.push(parts.0.parse::<i64>().unwrap());
        res.push(parts.1.parse::<i64>().unwrap());
    }else{
        res.push(stone * 2024);
    }

    res
}

pub fn run_task_1(data: String) {
    let init_arr = parse_input(data);
    let num_blink = 25;


    let (tx, rx) = mpsc::channel();
    let mut index = 0;
    for stone in init_arr {
        index += 1;
        let index = index.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let mut cur_stone_arr = Vec::new();
            cur_stone_arr.push(stone);

            for _ in 0..num_blink {
                let mut next_strone_arr = Vec::new();
                for stone in cur_stone_arr {
                    let processed_stone = process_stone(stone);
                    next_strone_arr.extend(processed_stone);
                }

                cur_stone_arr = next_strone_arr;
            }

            tx.send((index, cur_stone_arr)).unwrap();
        });
    }

    drop(tx);

    let mut values: Vec<(i32, Vec<i64>)> = Vec::new();
    for res in rx {
        values.push(res);
    }
    values.sort_by(|a, b| a.0.cmp(&b.0));

    let mut num_stones = 0;
    for val in values {
        for _ in val.1 {
            num_stones += 1;
        }
    }

    println!("Number of Stones: {}", num_stones);

    
    /*let grid = parse_input(data);
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

    println!("Sum: {}", sum);*/
}

pub fn run_task_2(data: String) {
    let init_arr = parse_input(data);
    let num_blink = 75;


    let (tx, rx) = mpsc::channel();
    let mut index = 0;
    for stone in init_arr {
        index += 1;
        let index = index.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let mut cur_stone_arr = Vec::new();
            cur_stone_arr.push(stone);

            for _ in 0..num_blink {
                let mut next_strone_arr = Vec::new();
                for stone in cur_stone_arr {
                    let processed_stone = process_stone(stone);
                    next_strone_arr.extend(processed_stone);
                }

                cur_stone_arr = next_strone_arr;
            }

            tx.send((index, cur_stone_arr)).unwrap();
        });
    }

    drop(tx);

    let mut values: Vec<(i32, Vec<i64>)> = Vec::new();
    for res in rx {
        values.push(res);
    }
    values.sort_by(|a, b| a.0.cmp(&b.0));

    let mut num_stones = 0;
    for val in values {
        for _ in val.1 {
            num_stones += 1;
        }
    }

    println!("Number of Stones: {}", num_stones);
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

fn parse_input(data: String) -> Vec<i64> {
    data.lines().next().unwrap().split(" ")
        .map(|val| val.parse().unwrap())
        .collect()
}
