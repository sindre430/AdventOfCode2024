use std::collections::{HashMap, HashSet, VecDeque};
mod structs;
use structs::plot::Plot;
use structs::region::Region;

pub fn run_task_1(data: String) {
    let map = parse_input(data);
    let mut regions = Vec::new();

    for (_, value) in &map {
        let char_regions = group_plots_to_regions(value.clone());
        regions.extend(char_regions);
    }

    let mut price = 0;
    for region in regions {
        price += region.get_price();
    }

    println!("Price: {}", price);
}

pub fn run_task_2(data: String) {
    let map = parse_input(data);
    let mut regions = Vec::new();

    for (_, value) in &map {
        let char_regions = group_plots_to_regions(value.clone());
        regions.extend(char_regions);
    }

    let mut price = 0;
    for region in regions {
        price += region.get_bulk_price();
    }

    println!("Price: {}", price);
}

fn group_plots_to_regions(plots: Vec<(i32, i32)>) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for &coord in &plots {
        if visited.contains(&coord) {
            continue;
        }

        let mut region = Region::new();
        let mut queue = VecDeque::new();
        queue.push_back(coord);

        while let Some(current) = queue.pop_front() {
            if !visited.insert(current) {
                continue;
            }

            let mut plot = Plot::new(current.0, current.1);

            let neighbors = [
                (current.0 + 1, current.1),
                (current.0 - 1, current.1),
                (current.0, current.1 + 1),
                (current.0, current.1 - 1),
            ];

            for neighbor in neighbors {
                if plots.contains(&neighbor) {
                    plot.neighbors.push(neighbor);
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }

            region.plots.push(plot);
        }

        regions.push(region);
    }

    regions
}

fn parse_input(data: String) -> HashMap<char, Vec<(i32, i32)>> {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if let Some(coords) = map.get_mut(&char) {
                coords.push((x as i32, y as i32));
            } else {
                let mut coords = Vec::new();
                coords.push((x as i32, y as i32));
                map.insert(char, coords);
            }
        }
    }

    map
}
