use std::collections::HashMap;

fn process_stone(stone: i64) -> Vec<i64> {
    let mut res = Vec::new();
    if stone == 0 {
        res.push(1);
        return res;
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let mid = stone_str.len() / 2;
        let parts = stone_str.split_at(mid);
        res.push(parts.0.parse::<i64>().unwrap());
        res.push(parts.1.parse::<i64>().unwrap());
    } else {
        res.push(stone * 2024);
    }

    res
}

fn blink(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones = HashMap::new();
    for (stone, count) in stones {
        let processed_stones = process_stone(stone);
        for processed_stone in processed_stones {
            let new_count = new_stones.get(&processed_stone).unwrap_or(&0) + count;
            new_stones.insert(processed_stone, new_count);
        }
    }

    new_stones
}

pub fn run_task_1(data: String) {
    let mut cur_stones = parse_input(data);

    for _ in 0..25 {
        cur_stones = blink(cur_stones);
    }

    let num_stones = cur_stones.values().sum::<i64>();
    println!("Number of Stones: {}", num_stones);
}

pub fn run_task_2(data: String) {
    let mut cur_stones = parse_input(data);

    for _ in 0..75 {
        cur_stones = blink(cur_stones);
    }

    let num_stones = cur_stones.values().sum::<i64>();
    println!("Number of Stones: {}", num_stones);
}

fn parse_input(data: String) -> HashMap<i64, i64> {
    let values: Vec<i64> = data
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|val| val.parse::<i64>().unwrap())
        .collect();

    let mut stones = HashMap::new();
    for val in values {
        let count = stones.get(&val).unwrap_or(&0) + 1;
        stones.insert(val, count);
    }

    stones
}
