use super::antenna::Antenna;

#[derive(Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub antennas: Vec<Antenna>,
    pub antinodes: Vec<(i32, i32)>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map {
            width,
            height,
            antennas: Vec::new(),
            antinodes: Vec::new(),
        }
    }

    pub fn add_antenna(&mut self, antenna: Antenna) {
        self.antennas.push(antenna);
    }

    pub fn add_antinode(&mut self, x: i32, y: i32) {
        self.antinodes.push((x, y));
    }

    pub fn add_antinodes(&mut self, antinodes: Vec<(i32, i32)>, allow_antanna_overlap: bool) {
        for antinode in antinodes.iter() {
            if antinode.0 < 0
                || antinode.1 < 0
                || antinode.0 >= self.width
                || antinode.1 >= self.height
            {
                continue;
            }

            if self.antinodes.contains(antinode) {
                continue;
            }

            if !allow_antanna_overlap {
                for antenna in self.antennas.iter() {
                    if antinode.0 == antenna.x && antinode.1 == antenna.y {
                        continue;
                    }
                }
            }

            self.antinodes.push(*antinode);
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut found = false;
                for antenna in self.antennas.iter() {
                    if antenna.x == x && antenna.y == y {
                        print!("{}", antenna.frequency);
                        found = true;
                        break;
                    }
                }

                if !found {
                    if self.antinodes.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }
}
