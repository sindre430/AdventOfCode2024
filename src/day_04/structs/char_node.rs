#[derive(Debug)]
pub struct CharNode<'a> {
    pub value: char,
    pub coordinates: (usize, usize),
    pub neighbors: Vec<&'a CharNode<'a>>,
}

impl<'a> CharNode<'a> {
    pub fn new(value: char, coordinates: (usize, usize)) -> CharNode<'a> {
        CharNode {
            value,
            coordinates,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: &'a CharNode<'a>) {
        self.neighbors.push(neighbor);
    }

    pub fn get_neighbors_by_value(&self, value: char) -> Vec<&CharNode<'a>> {
        self.neighbors
            .iter()
            .copied() // Dereference and re-reference for correct lifetime handling
            .filter(|node| node.value == value)
            .collect()
    }
}
