use std::collections::BTreeMap;

pub struct List {
    pub numbers_with_count: BTreeMap<i32, i32>,
}

impl List {
    pub fn new(numbers: Vec<i32>) -> Self {
        let mut numbers_with_count = BTreeMap::new();
        for number in numbers {
            let count = numbers_with_count.entry(number).or_insert(0);
            *count += 1;
        }

        List { numbers_with_count }
    }

    pub fn get_total_distance(&self, compare_list: List) -> i32 {
        let mut list_stack = Vec::<i32>::new();
        let mut compare_list_stack = Vec::<i32>::new();

        for (key, count) in &self.numbers_with_count {
            list_stack.extend(std::iter::repeat(key).take(*count as usize));
        }

        for (key, count) in &compare_list.numbers_with_count {
            compare_list_stack.extend(std::iter::repeat(key).take(*count as usize));
        }

        println!("List Stack: {:?}", list_stack);
        println!("Comparer List Stack: {:?}", compare_list_stack);

        let mut total_distance = 0;
        for i in 0..list_stack.len() {
            let distance = (list_stack[i] - compare_list_stack[i]).abs();
            println!("Distance: {}", distance);
            total_distance += distance;
        }

        total_distance
    }

    pub fn get_similarity_score(&self, compare_list: List) -> i32 {
        let mut similarity_score = 0;

        for (key, count) in &self.numbers_with_count {
            let similarity = compare_list.numbers_with_count.get(key);
            println!("Similarity: {:?}", similarity);
            println!("Count: {:?}", count);
            similarity_score +=
                count * key * compare_list.numbers_with_count.get(key).unwrap_or(&0);
        }

        similarity_score
    }
}
