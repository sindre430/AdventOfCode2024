#[derive(Clone)]
pub struct Antenna {
    pub x: i32,
    pub y: i32,
    pub frequency: char,
}

impl Antenna {
    pub fn new(x: i32, y: i32, frequency: char) -> Antenna {
        Antenna { x, y, frequency }
    }

    pub fn get_antinodes(&self, antenna: &Antenna) -> Vec<(i32, i32)> {
        let mut antinodes = Vec::new();
        let dx = antenna.x - self.x;
        let dy = antenna.y - self.y;

        antinodes.push((self.x - dx, self.y - dy));
        antinodes.push((antenna.x + dx, antenna.y + dy));

        antinodes
    }

    pub fn get_antinodes_full(
        &self,
        antenna: &Antenna,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> Vec<(i32, i32)> {
        let mut antinodes = Vec::new();
        let dx = antenna.x - self.x;
        let dy = antenna.y - self.y;

        let mut tmp_x = self.x;
        let mut tmp_y = self.y;
        loop {
            tmp_x -= dx;
            tmp_y -= dy;
            if tmp_x < min_x || tmp_y < min_y || tmp_x > max_x || tmp_y > max_y {
                break;
            }
            antinodes.push((tmp_x, tmp_y));
        }

        tmp_x = antenna.x;
        tmp_y = antenna.y;
        loop {
            tmp_x += dx;
            tmp_y += dy;
            if tmp_x < min_x || tmp_y < min_y || tmp_x > max_x || tmp_y > max_y {
                break;
            }
            antinodes.push((tmp_x, tmp_y));
        }

        antinodes.push((self.x, self.y));
        antinodes.push((antenna.x, antenna.y));

        antinodes
    }
}
