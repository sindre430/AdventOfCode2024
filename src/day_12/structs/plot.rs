pub struct Plot {
    pub x: i32,
    pub y: i32,
    pub neighbors: Vec<(i32, i32)>,
}

impl Plot {
    pub fn new(x: i32, y: i32) -> Plot {
        Plot {
            x,
            y,
            neighbors: Vec::new(),
        }
    }
}
