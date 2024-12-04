use std::{char, collections::HashMap};

use super::{char_node::CharNode, word::Word};

pub struct WordSearch {
    pub char_nodes: Vec<CharNode>,
}

impl WordSearch {
    pub fn new(lines: Vec<Vec<char>>) -> WordSearch {
        let mut char_nodes = Vec::new();

        // Process each character in the word search
        for (y, line) in lines.iter().enumerate() {
            for (x, &value) in line.iter().enumerate() {
                let coordinates = (x, y);
                let char_node = CharNode::new(value, coordinates);
                char_nodes.push(char_node);
            }
        }

        let word_search = WordSearch { char_nodes };

        word_search
    }

    pub fn search_for_word_x(&self, search_word: &str) -> Vec<Word> {
        if search_word.len() % 2 != 1 {
            panic!("Search word must have an odd number of characters");
        }

        let first_char = search_word.chars().next().unwrap();
        let start_nodes = self.get_char_nodes_by_value(first_char);

        let directions = [
            (-1, -1), // up-left
            (1, -1),  // up-right
            (-1, 1),  // down-left
            (1, 1),   // down-right
        ];

        let mut words = Vec::new();
        for node in &start_nodes {
            for direction in directions {
                let (dx, dy) = direction;
                let mut word = Word::new(node);

                let mut is_valid = true;
                for i in 1..search_word.len() {
                    let letter_x = node.coordinates.0 as isize + dx * i as isize;
                    let letter_y = node.coordinates.1 as isize + dy * i as isize;

                    // Ensure new coordinates are within bounds
                    if letter_x < 0
                        || letter_y < 0
                        || (letter_y as usize) >= self.char_nodes.len()
                        || (letter_x as usize) >= self.char_nodes.len()
                    {
                        is_valid = false;
                        break;
                    }

                    let search_char = search_word.chars().nth(i).unwrap();
                    let letter_node =
                        self.get_char_node_by_coordinates((letter_x as usize, letter_y as usize));

                    if letter_node.is_none() || letter_node.unwrap().value != search_char {
                        is_valid = false;
                        break;
                    }

                    word.add_coordinate(letter_node.unwrap());
                }

                if is_valid {
                    words.push(word);
                }
            }
        }

        // Find crossing words
        let center_char_index = (search_word.len() as f32 / 2_f32).floor() as usize;
        let mut center_coordinates = HashMap::new();
        for word in &words {
            *center_coordinates
                .entry(word.coordinates[center_char_index])
                .or_insert(0) += 1;
        }

        let crossing_coordinates: Vec<(usize, usize)> = center_coordinates
            .iter()
            .filter(|&(_, &count)| count > 1)
            .map(|(coordinates, _)| *coordinates)
            .collect();

        let crossing_words: Vec<Word> = words
            .iter()
            .filter(|w| crossing_coordinates.contains(&w.coordinates[center_char_index]))
            .cloned()
            .collect::<Vec<Word>>();

        crossing_words
    }

    pub fn search_for_word(&self, search_word: &str) -> Vec<Word> {
        let first_char = search_word.chars().next().unwrap();
        let start_nodes = self.get_char_nodes_by_value(first_char);

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

        let mut words = Vec::new();
        for node in &start_nodes {
            for direction in directions {
                let (dx, dy) = direction;
                let mut word = Word::new(node);

                let mut is_valid = true;
                for i in 1..search_word.len() {
                    let letter_x = node.coordinates.0 as isize + dx * i as isize;
                    let letter_y = node.coordinates.1 as isize + dy * i as isize;

                    // Ensure new coordinates are within bounds
                    if letter_x < 0
                        || letter_y < 0
                        || (letter_y as usize) >= self.char_nodes.len()
                        || (letter_x as usize) >= self.char_nodes.len()
                    {
                        is_valid = false;
                        break;
                    }

                    let search_char = search_word.chars().nth(i).unwrap();
                    let letter_node =
                        self.get_char_node_by_coordinates((letter_x as usize, letter_y as usize));

                    if letter_node.is_none() || letter_node.unwrap().value != search_char {
                        is_valid = false;
                        break;
                    }

                    word.add_coordinate(letter_node.unwrap());
                }

                if is_valid {
                    words.push(word);
                }
            }
        }

        words
    }

    pub fn get_char_node_by_coordinates(&self, coordinates: (usize, usize)) -> Option<&CharNode> {
        self.char_nodes
            .iter()
            .find(|&node| node.coordinates == coordinates)
    }

    pub fn get_char_nodes_by_value(&self, value: char) -> Vec<&CharNode> {
        self.char_nodes
            .iter()
            .filter(|&node| node.value == value)
            .collect()
    }
}
