mod structs;

use structs::word_search::WordSearch;

pub fn run_task_1(data: String) {
    let chars = parse_input(data);
    let word_search = WordSearch::new(chars);

    let search_word = "XMAS";
    let words = word_search.search_for_word(search_word);

    println!("Found {} instances of '{}'", words.len(), search_word);
}

pub fn run_task_2(data: String) {
    let chars = parse_input(data);
    let word_search = WordSearch::new(chars);

    let search_word = "MAS";
    let words = word_search.search_for_word_x(search_word);

    println!(
        "Found {} instances of '{}' in X shape",
        words.len() / 2,
        search_word
    );
}

fn parse_input(data: String) -> Vec<Vec<char>> {
    let chars = data.lines().map(|line| line.chars().collect()).collect();
    chars
}
