mod structs;
use std::char;
use std::sync::mpsc;
use std::thread;

use structs::map::Map;

pub fn run_task_1(data: String) {
    let (mut map, (width, height)) = parse_input(data);

    let mut guard_visited_positions: Vec<(i32, i32)> = Vec::new();
    loop {
        let new_position = map.move_guard();

        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= width
            || new_position.1 >= height
        {
            break;
        }

        let position_without_direction = (new_position.0, new_position.1);
        if !guard_visited_positions.contains(&position_without_direction) {
            guard_visited_positions.push(position_without_direction);
        }
    }

    println!("Visited positions: {}", guard_visited_positions.len());
}

pub fn run_task_2(data: String) {
    let (map, (width, height)) = parse_input(data);
    let (tx, rx) = mpsc::channel();

    for y in 0..height {
        for x in 0..width {
            let map_clone = map.clone();
            let tx = tx.clone();
            thread::spawn(move || {
                let mut loops = false;
                let cur_obstruction_location = (x, y);

                if map_clone.is_position_walkable(cur_obstruction_location) {
                    let mut map_clone = map_clone.clone();
                    map_clone.insert(cur_obstruction_location, '#');
                    let mut guard_visited_positions = Vec::new();

                    loop {
                        let new_position = map_clone.move_guard();

                        if new_position.0 < 0
                            || new_position.1 < 0
                            || new_position.0 >= width
                            || new_position.1 >= height
                        {
                            break;
                        }

                        if guard_visited_positions.contains(&new_position) {
                            loops = true;
                            break;
                        } else {
                            guard_visited_positions.push(new_position);
                        }
                    }
                }

                tx.send(loops).unwrap();
                println!("Thread done. Loops: {}", loops);
            });
        }
    }

    drop(tx);

    let mut possible_loop_obstructions = 0;
    for loops in rx {
        possible_loop_obstructions += if loops { 1 } else { 0 };
    }

    println!("Possible loop obstructions: {}", possible_loop_obstructions);
}

fn parse_input(data: String) -> (Map, (i32, i32)) {
    let mut map = Map::new();
    let mut width = 0;
    let mut y_index = 0;
    for line in data.lines() {
        let chars: Vec<char> = line.chars().collect();
        width = chars.len();
        for (x_index, c) in chars.iter().enumerate() {
            if *c == '^' {
                map.guard_position = (x_index as i32, y_index);
                map.insert(map.guard_position, '.');
                continue;
            }

            map.insert((x_index as i32, y_index), *c);
        }

        y_index += 1;
    }

    (map, (width as i32, y_index))
}
