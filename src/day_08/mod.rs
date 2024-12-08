mod structs;
use structs::{antenna::Antenna, map::Map};

pub fn run_task_1(data: String) {
    let mut map = parse_input(data);
    let antennas = map.clone().antennas;

    let mut checked_antennas: Vec<char> = Vec::new();
    for antenna in antennas.iter() {
        if checked_antennas.contains(&antenna.frequency) {
            continue;
        }

        let all_antennas_with_frequency: Vec<&Antenna> = antennas
            .iter()
            .filter(|a| a.frequency == antenna.frequency)
            .collect();

        let num_antennas = *&all_antennas_with_frequency.len();
        for a1 in 0..num_antennas - 1 {
            for a2 in a1 + 1..num_antennas {
                let ant1 = all_antennas_with_frequency[a1];
                let ant2 = all_antennas_with_frequency[a2];

                let antinodes = ant1.get_antinodes(ant2);
                map.add_antinodes(antinodes, false);
            }
        }

        checked_antennas.push(antenna.frequency);
    }

    map.print();

    println!("\nNum Antinodes: {}", map.antinodes.len());
}

pub fn run_task_2(data: String) {
    let mut map = parse_input(data);
    let antennas = map.clone().antennas;

    let mut checked_antennas: Vec<char> = Vec::new();
    for antenna in antennas.iter() {
        if checked_antennas.contains(&antenna.frequency) {
            continue;
        }

        let all_antennas_with_frequency: Vec<&Antenna> = antennas
            .iter()
            .filter(|a| a.frequency == antenna.frequency)
            .collect();

        let num_antennas = *&all_antennas_with_frequency.len();
        for a1 in 0..num_antennas - 1 {
            for a2 in a1 + 1..num_antennas {
                let ant1 = all_antennas_with_frequency[a1];
                let ant2 = all_antennas_with_frequency[a2];

                let antinodes = ant1.get_antinodes_full(ant2, 0, 0, map.width, map.height);
                map.add_antinodes(antinodes, true);
            }
        }

        checked_antennas.push(antenna.frequency);
    }

    map.print();

    println!("\nNum Antinodes: {}", map.antinodes.len());
}

fn parse_input(data: String) -> Map {
    let height = data.lines().count();
    let width = data.lines().next().unwrap().chars().count();
    let mut map = Map::new(width as i32, height as i32);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                map.add_antenna(Antenna::new(x as i32, y as i32, c));
            }
            if c == '#' {
                map.add_antinode(x as i32, y as i32);
            }
        }
    }

    map
}
