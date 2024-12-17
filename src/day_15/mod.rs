use std::collections::{HashMap, VecDeque};

pub fn run_task_1(data: String) {
    let (mut map, mut robot, mut moves) = parse_input(data, false);

    while let Some(dir) = moves.pop_front() {
        let next_pos = match dir {
            1 => (robot.0, robot.1 - 1),
            2 => (robot.0 + 1, robot.1),
            3 => (robot.0, robot.1 + 1),
            4 => (robot.0 - 1, robot.1),
            _ => (0, 0),
        };

        let next_tile = map.get(&next_pos).unwrap();
        if *next_tile == 2 {
            // # - Wall
            continue;
        }

        if *next_tile == 0 {
            // . - Free space
            robot = next_pos;
            continue;
        }

        // Box
        let mut next_free_spot = next_pos;
        loop {
            next_free_spot = match dir {
                1 => (next_free_spot.0, next_free_spot.1 - 1),
                2 => (next_free_spot.0 + 1, next_free_spot.1),
                3 => (next_free_spot.0, next_free_spot.1 + 1),
                4 => (next_free_spot.0 - 1, next_free_spot.1),
                _ => (0, 0),
            };

            let next_tile = map.get(&next_free_spot).unwrap();
            if *next_tile == 2 {
                // # - Wall
                break;
            }

            if *next_tile == 1 {
                // O - Box
                continue;
            }

            // . - Free space
            map.insert(next_pos, 0);
            map.insert(next_free_spot, 1);

            robot = next_pos;
            break;
        }
    }

    let sum_boxes: i32 = map
        .iter()
        .filter(|&(_, &v)| v == 1)
        .map(|(&(x, y), _)| x + 100 * y)
        .sum();

    println!("Sum of boxes: {}", sum_boxes);
}

pub fn run_task_2(data: String) {
    let (mut map, mut robot, mut moves) = parse_input(data, true);

    while let Some(dir) = moves.pop_front() {
        let next_pos = match dir {
            1 => (robot.0, robot.1 - 1),
            2 => (robot.0 + 1, robot.1),
            3 => (robot.0, robot.1 + 1),
            4 => (robot.0 - 1, robot.1),
            _ => (0, 0),
        };

        let next_tile = map.get(&next_pos).unwrap();
        if *next_tile == 2 {
            // # - Wall
            continue;
        }

        if *next_tile == 0 {
            // . - Free space
            robot = next_pos;
            continue;
        }

        // Box
        let mut affected_tiles: HashMap<(i32, i32), i32> = HashMap::new();
        if *next_tile == 1 {
            // [
            affected_tiles.insert(next_pos, 1);
            affected_tiles.insert((next_pos.0 + 1, next_pos.1), 3);
        } else {
            // ]
            affected_tiles.insert(next_pos, 3);
            affected_tiles.insert((next_pos.0 - 1, next_pos.1), 1);
        }

        let mut move_robot = true;
        let mut tiles_to_check = affected_tiles.clone();
        loop {
            let mut new_tiles_to_check: HashMap<(i32, i32), i32> = HashMap::new();

            for tile in &tiles_to_check {
                let next_tile_pos = match dir {
                    1 => (tile.0 .0, tile.0 .1 - 1),
                    2 => (tile.0 .0 + 1, tile.0 .1),
                    3 => (tile.0 .0, tile.0 .1 + 1),
                    4 => (tile.0 .0 - 1, tile.0 .1),
                    _ => (0, 0),
                };

                let next_tile = map.get(&next_tile_pos).unwrap();

                if *next_tile == 2 {
                    // # - Wall
                    move_robot = false;
                    affected_tiles.clear();
                    new_tiles_to_check.clear();
                    break;
                }

                if *next_tile == 0 {
                    continue;
                }

                if affected_tiles.contains_key(&next_tile_pos) {
                    continue;
                }

                if *next_tile == 1 {
                    // [
                    new_tiles_to_check.insert(next_tile_pos, 1);
                    new_tiles_to_check.insert((next_tile_pos.0 + 1, next_tile_pos.1), 3);
                } else {
                    // ]
                    new_tiles_to_check.insert(next_tile_pos, 3);
                    new_tiles_to_check.insert((next_tile_pos.0 - 1, next_tile_pos.1), 1);
                }
            }

            if new_tiles_to_check.is_empty() {
                break;
            }

            affected_tiles.extend(new_tiles_to_check.clone());

            tiles_to_check = new_tiles_to_check;
        }

        // Set affected tiles to 0 (.)
        for tile in &affected_tiles {
            map.insert(*tile.0, 0);
        }

        // Move affected tiles
        for tile in &affected_tiles {
            let next_tile = match dir {
                1 => (tile.0 .0, tile.0 .1 - 1),
                2 => (tile.0 .0 + 1, tile.0 .1),
                3 => (tile.0 .0, tile.0 .1 + 1),
                4 => (tile.0 .0 - 1, tile.0 .1),
                _ => (0, 0),
            };

            map.insert(next_tile, *tile.1);
        }

        if move_robot {
            robot = next_pos;
        }
    }

    let sum_boxes: i32 = map
        .iter()
        .filter(|&(_, &v)| v == 1)
        .map(|(&(x, y), _)| x + 100 * y)
        .sum();

    println!("Sum of boxes: {}", sum_boxes);
}

fn parse_input(
    data: String,
    double_width: bool,
) -> (HashMap<(i32, i32), i32>, (i32, i32), VecDeque<i32>) {
    let mut map = HashMap::<(i32, i32), i32>::new();
    let mut robot = (0, 0);
    let mut moves = VecDeque::new();

    let mut process_map = true;
    for (y, line) in data.lines().enumerate() {
        if line.is_empty() {
            process_map = false;
        }

        if process_map {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        if double_width {
                            map.insert((x as i32 * 2, y as i32), 0);
                            map.insert((x as i32 * 2 + 1, y as i32), 0);
                        } else {
                            map.insert((x as i32, y as i32), 0);
                        }
                    }
                    'O' => {
                        if double_width {
                            map.insert((x as i32 * 2, y as i32), 1);
                            map.insert((x as i32 * 2 + 1, y as i32), 3);
                        } else {
                            map.insert((x as i32, y as i32), 1);
                        }
                    }
                    '#' => {
                        if double_width {
                            map.insert((x as i32 * 2, y as i32), 2);
                            map.insert((x as i32 * 2 + 1, y as i32), 2);
                        } else {
                            map.insert((x as i32, y as i32), 2);
                        }
                    }
                    '@' => {
                        if double_width {
                            robot = (x as i32 * 2, y as i32);
                            map.insert((x as i32 * 2, y as i32), 0);
                            map.insert((x as i32 * 2 + 1, y as i32), 0);
                        } else {
                            robot = (x as i32, y as i32);
                            map.insert((x as i32, y as i32), 0);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            for c in line.chars() {
                match c {
                    '^' => {
                        moves.push_back(1);
                    }
                    '>' => {
                        moves.push_back(2);
                    }
                    'v' => {
                        moves.push_back(3);
                    }
                    '<' => {
                        moves.push_back(4);
                    }
                    _ => {}
                }
            }
        }
    }

    (map, robot, moves)
}
