use std::collections::HashMap;

use regex::Regex;

pub fn run_task_1(data: String) {
    let robots = parse_input(data);

    let width = 101; // Test data = 11
    let height = 103; // Test data = 7
    let seconds = 100;

    let mid_horizontal = height / 2;
    let mid_vertical = width / 2;

    let mut map = HashMap::<(i32, i32), i32>::new();
    for robot in &robots {
        let x = (robot[0] + seconds * robot[2]).rem_euclid(width);
        let y = (robot[1] + seconds * robot[3]).rem_euclid(height);

        // Ignore for middle lines
        if x == mid_vertical || y == mid_horizontal {
            continue;
        }

        let num = map.get(&(x, y)).unwrap_or(&0) + 1;

        map.insert((x, y), num);
    }

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for entry in map.iter() {
        match (entry.0 .0 < mid_vertical, entry.0 .1 < mid_horizontal) {
            (true, true) => q1 += entry.1,
            (false, true) => q2 += entry.1,
            (true, false) => q3 += entry.1,
            (false, false) => q4 += entry.1,
        }
    }

    let sum = q1 * q2 * q3 * q4;

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {
    let robots = parse_input(data);

    let width = 101;
    let height = 103;

    let mut seconds = 0;

    let mut finished_map = HashMap::<(i32, i32), i32>::new();

    loop {
        seconds += 1;

        let mut map = HashMap::<(i32, i32), i32>::new();
        for robot in &robots {
            let x = (robot[0] + seconds * robot[2]).rem_euclid(width);
            let y = (robot[1] + seconds * robot[3]).rem_euclid(height);

            let num = map.get(&(x, y)).unwrap_or(&0) + 1;

            // Only allow one robot per position
            if num > 1 {
                break;
            }

            map.insert((x, y), num);
        }

        if map.keys().len() == robots.len() {
            println!("Seconds: {}", seconds);
            finished_map = map;
            break;
        }
    }

    for y in 0..height {
        for x in 0..width {
            if finished_map.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input(data: String) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        res.push(vec![
            caps[1].parse::<i32>().unwrap(),
            caps[2].parse::<i32>().unwrap(),
            caps[3].parse::<i32>().unwrap(),
            caps[4].parse::<i32>().unwrap(),
        ]);
    }

    res
}
