use super::char_node::CharNode;

pub struct Word<'a> {
    pub char_nodes: Vec<&'a CharNode<'a>>,
}

impl<'a> Word<'a> {
    pub fn new(start_char_node: &'a CharNode<'a>) -> Word<'a> {
        Word {
            char_nodes: vec![start_char_node]
        }
    }

    pub fn add_char_node(&mut self, char_node: &'a CharNode<'a>) {
        self.char_nodes.push(char_node);
    }
}
