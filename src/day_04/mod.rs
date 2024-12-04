mod structs;

use structs::word_search::WordSearch;

pub fn run_task_1(data: String) {
    let chars = parse_input(data);
    let wordSearch = WordSearch::new(chars);

    let x_chars = wordSearch.get_char_nodes_by_value('X');

    for char_node in x_chars {
        println!(
            "{:?} {} {}",
            char_node.coordinates,
            char_node.value,
            char_node
                .neighbors
                .clone()
                .into_iter()
                .map(|node| node.value)
                .collect::<Vec<char>>()
                .iter()
                .collect::<String>()
        );
    }
}

pub fn run_task_2(data: String) {}

fn parse_input(data: String) -> Vec<Vec<char>> {
    let chars = data.lines().map(|line| line.chars().collect()).collect();
    chars
}
