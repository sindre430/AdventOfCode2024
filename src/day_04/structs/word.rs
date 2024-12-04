use super::char_node::CharNode;

#[derive(Clone)]
pub struct Word {
    pub coordinates: Vec<(usize, usize)>,
}

impl Word {
    pub fn new(start_char_node: &CharNode) -> Word {
        Word {
            coordinates: vec![start_char_node.coordinates],
        }
    }

    pub fn add_coordinate(&mut self, char_node: &CharNode) {
        self.coordinates.push(char_node.coordinates);
    }
}
