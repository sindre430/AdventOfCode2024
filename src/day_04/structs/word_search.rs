use super::char_node::CharNode;

pub struct WordSearch<'a> {
    pub char_nodes: Vec<CharNode<'a>>,
}

impl<'a> WordSearch<'a> {
    pub fn new(lines: Vec<Vec<char>>) -> WordSearch<'a> {
        let mut char_nodes = Vec::new();

        // Find puzzle dimensions
        let width = lines[0].len();
        let height = lines.len();

        // Process each character in the word search
        for (y, line) in lines.iter().enumerate() {
            for (x, &value) in line.iter().enumerate() {
                let coordinates = (x, y);
                let char_node = CharNode::new(value, coordinates);
                char_nodes.push(char_node);
            }
        }

        let mut word_search = WordSearch { char_nodes };
        // let char_nodes = word_search.get_char_nodes();

        // Add neighbors to each char node
        let char_nodes_ptr = word_search.char_nodes.as_mut_ptr(); // Get raw pointer to avoid borrowing conflicts

        for i in 0..word_search.char_nodes.len() {
            let (x, y) = unsafe { (*char_nodes_ptr.add(i)).coordinates };
            let directions = [
                (-1, 0),  // left
                (1, 0),   // right
                (0, -1),  // up
                (0, 1),   // down
                (-1, -1), // up-left
                (1, -1),  // up-right
                (-1, 1),  // down-left
                (1, 1),   // down-right
            ];

            for (dx, dy) in directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                // Ensure new coordinates are within bounds
                if new_x >= 0 && new_y >= 0 && (new_y as usize) < height && (new_x as usize) < width
                {
                    let neighbor_index = new_y as usize * width + new_x as usize;
                    let neighbor = unsafe { &mut *char_nodes_ptr.add(neighbor_index) };
                    unsafe { (*char_nodes_ptr.add(i)).add_neighbor(neighbor) };
                }
            }
        }

        word_search
    }

    pub fn search_for_word(
        &self,
        search_word: &str,
        char_nodes: Option<Vec<&CharNode>>,
    ) -> Vec<&CharNode> {
        let first_char = search_word.chars().next().unwrap();

        // Search for first char nodes if none are provided
        if !char_nodes.is_some() {
            let first_char_nodes = self.get_char_nodes_by_value(first_char);

            return self.search_for_word(search_word.split_at(1).1, Some(first_char_nodes));
        }

        // Return all char nodes if search word is empty
        if search_word.len() == 0 {
            return char_nodes.unwrap();
        }

        let first_char_nodes = match char_nodes {
            Some(nodes) => nodes,
            None => self.get_char_nodes_by_value(first_char),
        };

        /*let first_char = search_word.chars().next().unwrap();
        let first_char_nodes = self.get_char_nodes_by_value(first_char);

        let mut found_words = Vec::new();
        for node in first_char_nodes {
            let mut visited = vec![false; self.char_nodes.len()];
            let mut word = String::new();
            word.push(node.value);

            if self.search_word_recursive(node, search_word, &mut visited, &mut word) {
                found_words.push(node);
            }
        }

        found_words*/
    }

    fn get_char_nodes(&mut self) -> Vec<&mut CharNode<'a>> {
        self.char_nodes.iter_mut().collect()
    }

    pub fn get_char_node_by_coordinates(&self, coordinates: (usize, usize)) -> &CharNode {
        self.char_nodes
            .iter()
            .find(|&node| node.coordinates == coordinates)
            .unwrap()
    }

    pub fn get_char_nodes_by_value(&self, value: char) -> Vec<&CharNode> {
        self.char_nodes
            .iter()
            .filter(|&node| node.value == value)
            .collect()
    }
}
