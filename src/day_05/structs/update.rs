use super::ordering_rule::OrderingRule;

pub struct Update {
    pub page_numbers: Vec<i32>,
}

impl Update {
    pub fn new(page_numbers: Vec<i32>) -> Update {
        Update { page_numbers }
    }

    pub fn get_middle_page(&self) -> i32 {
        let middle_index = self.page_numbers.len() / 2;
        self.page_numbers[middle_index]
    }

    pub fn validate_ordering(&self, ordering_rules: &Vec<OrderingRule>) -> bool {
        let mut is_correct_ordered = true;
        for rule in ordering_rules.iter() {
            if !self.page_numbers.contains(&rule.left) || !self.page_numbers.contains(&rule.right) {
                continue;
            }

            let left_index = self
                .page_numbers
                .iter()
                .position(|&x| x == rule.left)
                .unwrap();
            let right_index = self
                .page_numbers
                .iter()
                .position(|&x| x == rule.right)
                .unwrap();
            if left_index > right_index {
                is_correct_ordered = false;
                break;
            }
        }

        is_correct_ordered
    }

    pub fn correct_ordering(&self, ordering_rules: &Vec<OrderingRule>) -> Vec<i32> {
        let mut correct_ordering = Vec::new();

        correct_ordering
    }
}
