use std::{
    cmp::Reverse,
    collections::hash_map::DefaultHasher,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use priority_queue::PriorityQueue;

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    position: (usize, usize),
    direction: usize,
    history: Vec<(usize, usize)>,
}

pub fn run_task_1(data: String) {
    let (start, end, map) = parse_input(data);

    let (_, cost, _) = find_best_path(start, end, map.clone(), i32::MAX);

    println!("Result: {}", cost);
}

pub fn run_task_2(data: String) {
    let (start, end, map) = parse_input(data);

    let mut best_cost = i32::MAX;
    let mut best_tiles = HashSet::new();
    let mut maps_to_test = vec![map.clone()];
    let mut maps_memory = HashSet::new();
    loop {
        let mut new_maps_to_test = vec![];

        for map in maps_to_test.iter() {
            let (state, cost, skipped_tiles) = find_best_path(start, end, map.clone(), best_cost);

            // Break if we can't find a better path
            if cost > best_cost {
                continue;
            }
            best_cost = cost;

            // Process skipped tiles
            let mut prev_tile: (usize, usize) = (0, 0);
            for history in state.history.iter() {
                if skipped_tiles.contains(history) {
                    let mut new_map = map.clone();
                    new_map.insert(prev_tile, false);

                    let map_hash = hash_hashmap(&new_map);
                    if maps_memory.contains(&map_hash) {
                        continue;
                    }
                    maps_memory.insert(map_hash);

                    new_maps_to_test.push(new_map);
                }
                prev_tile = *history;
            }

            // Add all tiles to best_tiles
            for tile in state.history {
                best_tiles.insert(tile);
            }
        }

        if new_maps_to_test.is_empty() {
            break;
        }

        maps_to_test = new_maps_to_test;
    }

    println!("Result: {}", best_tiles.len());
}

fn find_best_path(
    start: (usize, usize),
    end: (usize, usize),
    map: HashMap<(usize, usize), bool>,
    max_cost: i32,
) -> (State, i32, HashSet<(usize, usize)>) {
    let mut pq: PriorityQueue<State, Reverse<i32>> = PriorityQueue::new();
    let mut visited = HashSet::new();

    pq.push(
        State {
            position: start,
            direction: 1, // East
            history: vec![start],
        },
        Reverse(0),
    );

    let mut best_cost = i32::MAX;
    let mut best_state = State {
        position: (0, 0),
        direction: 0,
        history: vec![],
    };
    let mut skipped_tiles: HashSet<(usize, usize)> = HashSet::new();
    while let Some((state, Reverse(cost))) = pq.pop() {
        if cost > max_cost || (state.position.0, state.position.1) == end {
            best_state = state.clone();
            best_cost = cost;
            break;
        }

        // Skip if already visited
        if visited.contains(&state.position) {
            skipped_tiles.insert(state.position);
            continue;
        }
        visited.insert(state.position);

        // Move Straight, Right, Left
        for &dir in &[0, 1, 3] {
            let mut new_state = state.clone();

            let dir_cost = if dir != 0 { 1000 } else { 0 };
            let new_dir = (new_state.direction + dir) % 4;
            let (dx, dy) = DIRECTIONS[new_dir];

            let new_x = (new_state.position.0 as i32 + dx) as usize;
            let new_y = (new_state.position.1 as i32 + dy) as usize;

            if map[&(new_x, new_y)] {
                new_state.position = (new_x, new_y);
                new_state.direction = new_dir;
                new_state.history.push((new_x, new_y));
                pq.push(new_state, Reverse(cost + 1 + dir_cost));
            }
        }
    }

    (best_state, best_cost, skipped_tiles)
}

fn hash_hashmap<K, V>(map: &HashMap<K, V>) -> u64
where
    K: Hash + Ord,
    V: Hash,
{
    let mut hasher = DefaultHasher::new();

    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by_key(|&(k, _)| k);

    for (key, value) in entries {
        key.hash(&mut hasher);
        value.hash(&mut hasher);
    }

    hasher.finish()
}

fn parse_input(
    data: String,
) -> (
    (usize, usize),
    (usize, usize),
    HashMap<(usize, usize), bool>,
) {
    let mut map = HashMap::<(usize, usize), bool>::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c != '#');
            if c == 'S' {
                start = (x, y);
            } else if c == 'E' {
                end = (x, y);
            }
        }
    }

    (start, end, map)
}
