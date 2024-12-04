pub struct CharNode {
    pub value: char,
    pub coordinates: (usize, usize),
}

impl CharNode {
    pub fn new(value: char, coordinates: (usize, usize)) -> CharNode {
        CharNode { value, coordinates }
    }
}
