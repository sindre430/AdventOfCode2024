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

    pub fn correct_ordering(input_order: &Vec<i32>, ordering_rules: &Vec<OrderingRule>) -> Vec<i32> {
        let rules_of_interest = ordering_rules
        .iter()
        .filter(|rule| input_order.contains(&rule.left) && input_order.contains(&rule.right))
        .cloned()
        .collect::<Vec<OrderingRule>>();

        let mut rules_for_numbers: Vec<(i32, Vec<&OrderingRule>)> = Vec::new();
        for (_, num) in input_order.iter().enumerate() {
            let rules = rules_of_interest
                .iter()
                .filter(|rule| rule.right == *num)
                .collect::<Vec<&OrderingRule>>();

            rules_for_numbers.push((*num, rules));
        }

        rules_for_numbers
            .sort_by(|a, b| {
                a.1.len().cmp(&b.1.len())
            });

        let corrected_ordering= rules_for_numbers
            .iter()
            .map(|rfn| rfn.0 )
            .collect::<Vec<i32>>();

        corrected_ordering
    }
}
