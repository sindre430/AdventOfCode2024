use structs::{ordering_rule::OrderingRule, update::Update};

mod structs;

pub fn run_task_1(data: String) {
    let (ordering_rules, updates) = parse_input(data);

    let mut sum = 0;
    for update in updates.iter() {
        let is_correct_ordered = update.validate_ordering(&ordering_rules);
        if is_correct_ordered {
            sum += update.get_middle_page();
        }
    }

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {}

fn parse_input(data: String) -> (Vec<OrderingRule>, Vec<Update>) {
    let mut ordering_rules = Vec::new();
    let mut updates = Vec::new();

    let mut processing_ordering_rules = true;
    for line in data.lines() {
        if line.is_empty() {
            processing_ordering_rules = false;
            continue;
        }

        if processing_ordering_rules {
            let values = line.split("|").collect::<Vec<&str>>();
            ordering_rules.push(OrderingRule::new(
                values[0].parse().unwrap(),
                values[1].parse().unwrap(),
            ));
        } else {
            let values = line
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|val| val.parse().unwrap())
                .collect::<Vec<i32>>();
            updates.push(Update::new(values));
        }
    }

    (ordering_rules, updates)
}
