mod structs;
use structs::disk_block::DiskBlock;
use structs::disk_block::FileBlock;
use structs::disk_block::FreeSpace;

pub fn run_task_1(data: String) {
    let mut diskmap = parse_input(data);

    let mut prev_free_space_index = 0;
    let mut swap_indexes = Vec::new();
    for (rev_index, c) in diskmap.iter().rev().enumerate() {
        if c.as_any().is::<FreeSpace>() {
            continue;
        }

        match find_next_free_space_index(&diskmap, prev_free_space_index) {
            Some(free_space_index) => {
                prev_free_space_index = free_space_index;
            }
            None => {
                break;
            }
        }

        let index = diskmap.len() - rev_index - 1;
        if index <= prev_free_space_index {
            break;
        }

        swap_indexes.push((index, prev_free_space_index));
        prev_free_space_index += 1;
    }

    for (from, to) in swap_indexes {
        diskmap.swap(from, to);
    }

    let mut sum: u64 = 0;
    for (index, c) in diskmap.iter().enumerate() {
        if let Some(file_block) = c.as_any().downcast_ref::<FileBlock>() {
            sum += index as u64 * file_block.id as u64;
        }
    }

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {}

fn find_next_free_space_index(
    disk_map: &Vec<Box<dyn DiskBlock>>,
    start_index: usize,
) -> Option<usize> {
    for (i, block) in disk_map.iter().enumerate().skip(start_index) {
        if block.as_any().is::<FreeSpace>() {
            return Some(i);
        }
    }

    None
}

fn parse_input(data: String) -> Vec<Box<dyn DiskBlock>> {
    let raw_data = data.lines().next().unwrap_or("");
    let mut result: Vec<Box<dyn DiskBlock>> = Vec::new();
    let mut is_file_block = true;
    let mut file_block_id = 0;
    for c in raw_data.chars() {
        let count = c.to_digit(10).unwrap_or(0) as usize;
        if is_file_block {
            for _ in 0..count {
                result.push(Box::new(FileBlock { id: file_block_id }));
            }
            file_block_id += 1;
        } else {
            for _ in 0..count {
                result.push(Box::new(FreeSpace {}));
            }
        }

        is_file_block = !is_file_block;
    }

    result
}
