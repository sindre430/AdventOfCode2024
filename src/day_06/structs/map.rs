use std::collections::HashMap;

pub struct Map {
    pub positions: HashMap<(i32, i32), char>,
    pub guard_position: (i32, i32),
    pub guard_direction: char,
}

impl Map {
    pub fn new() -> Map {
        Map {
            positions: HashMap::new(),
            guard_position: (0, 0),
            guard_direction: 'U',
        }
    }

    pub fn clone(&self) -> Map {
        let mut new_map = Map::new();

        for (coordinates, value) in self.positions.iter() {
            new_map.insert(*coordinates, *value);
        }

        new_map.guard_position = self.guard_position;
        new_map.guard_direction = self.guard_direction;

        new_map
    }

    pub fn insert(&mut self, coordinates: (i32, i32), value: char) {
        self.positions.insert(coordinates, value);
    }

    pub fn is_position_walkable(&self, coordinates: (i32, i32)) -> bool {
        match self.positions.get(&coordinates) {
            Some(value) => *value != '#',
            None => true,
        }
    }

    pub fn move_guard(&mut self) -> (i32, i32, char) {
        let (x, y) = self.guard_position;
        let (new_x, new_y) = match self.guard_direction {
            'U' => (x, y - 1),
            'D' => (x, y + 1),
            'L' => (x - 1, y),
            'R' => (x + 1, y),
            _ => panic!("Invalid guard direction"),
        };

        if !self.is_position_walkable((new_x, new_y)) {
            self.turn_guard();
            return self.move_guard();
        }

        self.guard_position = (new_x, new_y);

        (new_x, new_y, self.guard_direction)
    }

    pub fn turn_guard(&mut self) {
        self.guard_direction = match self.guard_direction {
            'U' => 'R',
            'R' => 'D',
            'D' => 'L',
            'L' => 'U',
            _ => panic!("Invalid guard direction"),
        };
    }
}
