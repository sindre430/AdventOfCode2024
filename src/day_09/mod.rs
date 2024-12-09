pub fn run_task_1(data: String) {
    let mut diskmap = parse_input(data);

    println!("Diskmap: {:?}", diskmap);

    let mut prev_free_space_index = 0;
    let mut swap_indexes = Vec::new();
    for (rev_index, c) in diskmap.iter().rev().enumerate() {
        if *c == '-' {
            break;
        }

        if *c == '.' {
            continue;
        }

        match find_char_from_index(&diskmap, '.', prev_free_space_index) {
            Some(free_space_index) => {
                println!("Found free space at index: {}", free_space_index);
                prev_free_space_index = free_space_index;
            }
            None => {
                println!("No more free space found");
                break;
            }
        }

        swap_indexes.push((diskmap.len() - rev_index - 1, prev_free_space_index));
        println!(
            "Swapping {} with {}",
            diskmap.len() - rev_index - 1,
            prev_free_space_index
        );
        prev_free_space_index += 1;
    }

    for (from, to) in swap_indexes {
        diskmap.swap(from, to);
    }

    for c in diskmap {
        print!("{}", c);
    }
}

pub fn run_task_2(data: String) {}

fn find_char_from_index(vec: &Vec<char>, target: char, start_index: usize) -> Option<usize> {
    // Check if the start_index is within the bounds of the Vec
    if start_index >= vec.len() {
        return None;
    }

    // Iterate over the vector starting from the given index
    for (i, &c) in vec.iter().enumerate().skip(start_index) {
        if c == target {
            return Some(i);
        }
    }
    None
}

fn parse_input(data: String) -> Vec<char> {
    let raw_data = data.lines().next().unwrap_or("");
    let mut result: Vec<char> = Vec::new();
    let mut is_file_block = true;
    let mut file_block_id = 0;
    for c in raw_data.chars() {
        let count = c.to_digit(10).unwrap_or(0) as usize;
        if is_file_block {
            let block = file_block_id.to_string().repeat(count);
            result.extend(block.chars());
            file_block_id += 1;
        } else {
            let dots = ".".repeat(count);
            result.extend(dots.chars());
        }

        is_file_block = !is_file_block;
    }

    result
}
