use std::char;

use super::{char_node::{self, CharNode}, word::Word};

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

    pub fn search_for_word<'b>(
        &'b self,
        search_word: &str,
        mut char_nodes: Option<Vec<&'b CharNode<'b>>>,
    ) -> Option<Vec<Word<'b>>> {
        if search_word.is_empty() {
            // Return None if the search word is empty
            return None;
        }
    
        let first_char = search_word.chars().next().unwrap();
        let mut remaining_word = if search_word.is_empty() {
            ""
        } else {
            &search_word[1..]
        };

        // Initialize char_nodes if None
        if char_nodes.is_none() {
            let first_nodes = self.get_char_nodes_by_value(first_char);
            println!("First nodes: {:#?}", first_nodes.len());
            char_nodes = Some(first_nodes);
            remaining_word = if remaining_word.is_empty() {
                ""
            } else {
                &remaining_word[1..]
            };
        }
    
        let mut words = Vec::new();
    
        for char_node in char_nodes.unwrap() {
            let matching_neighbors = char_node.get_neighbors_by_value(first_char);
    
            if matching_neighbors.is_empty() {
                // If no neighbors, create a new Word and add it
                words.push(Word::new(char_node));
            } else {
                // Recursively search the remaining word in matching neighbors
                if let Some(mut found_words) = self.search_for_word(remaining_word, Some(matching_neighbors)) {
                    for word in &mut found_words {
                        word.add_char_node(char_node);
                    }
                    words.extend(found_words);
                }
            }
        }
    
        if words.is_empty() {
            None
        } else {
            Some(words)
        }
    }
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
