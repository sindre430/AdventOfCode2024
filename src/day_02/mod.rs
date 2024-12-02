mod structs;

use structs::report::Report;

pub fn run_task_1(data: String) {
    let reports = parse_input(data);

    let mut number_of_safe_reports = 0;
    for report in &reports {
        let is_safe = report.check_if_safe(3, 0);
        if is_safe {
            number_of_safe_reports += 1;
        }
        println!(
            "{}: {}",
            report.levels_to_string(),
            if is_safe { "Safe" } else { "Unsafe" }
        );
    }

    println!("Number of Safe Reports: {}", number_of_safe_reports);
}

pub fn run_task_2(data: String) {
    let reports = parse_input(data);

    let mut number_of_safe_reports = 0;
    for report in &reports {
        let is_safe = report.check_if_safe(3, 1);
        if is_safe {
            number_of_safe_reports += 1;
        }
        println!(
            "{}: {}",
            report.levels_to_string(),
            if is_safe { "Safe" } else { "Unsafe" }
        );
    }

    println!("Number of Safe Reports: {}", number_of_safe_reports);
}

fn parse_input(data: String) -> Vec<Report> {
    let reports = data
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            Report::new(numbers)
        })
        .collect();

    reports
}
