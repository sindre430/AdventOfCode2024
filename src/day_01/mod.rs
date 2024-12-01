mod structs;
use structs::list::List;

pub fn run_task_1(data: String) {
    let (raw_list_1, raw_list_2) = parse_input(data);

    let list_1 = List::new(raw_list_1);
    let list_2 = List::new(raw_list_2);

    let total_distance = list_1.get_total_distance(list_2);

    println!("Total Distance: {}", total_distance);
}

pub fn run_task_2(data: String) {
    let (raw_list_1, raw_list_2) = parse_input(data);

    let list_1 = List::new(raw_list_1);
    let list_2 = List::new(raw_list_2);

    let similarity_score = list_1.get_similarity_score(list_2);

    println!("Similarity Score: {}", similarity_score);
}

fn parse_input(data: String) -> (Vec<i32>, Vec<i32>) {
    data.lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .unzip()
}
