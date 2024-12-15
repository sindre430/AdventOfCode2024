use std::env;
use std::path::Path;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

macro_rules! measure_time {
    ($func:expr, $data:expr) => {{
        println!("----------------------------");
        let start = Instant::now();
        let result = $func($data);
        let duration = start.elapsed();
        println!(
            "----------------------------\nExecution time: {:?}",
            duration
        );
        result
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (day, task, is_test) = handle_arguments(&args);

    println!(
        "Executing Program{}",
        if is_test { " (in Test Mode)" } else { "" }
    );
    println!("~ Day: {}", day);
    println!("~ Task: {}", task);

    let task_func = match day.as_str() {
        "01" => Some(if task == "1" {
            day_01::run_task_1
        } else {
            day_01::run_task_2
        }),
        "02" => Some(if task == "1" {
            day_02::run_task_1
        } else {
            day_02::run_task_2
        }),
        "03" => Some(if task == "1" {
            day_03::run_task_1
        } else {
            day_03::run_task_2
        }),
        "04" => Some(if task == "1" {
            day_04::run_task_1
        } else {
            day_04::run_task_2
        }),
        "05" => Some(if task == "1" {
            day_05::run_task_1
        } else {
            day_05::run_task_2
        }),
        "06" => Some(if task == "1" {
            day_06::run_task_1
        } else {
            day_06::run_task_2
        }),
        "07" => Some(if task == "1" {
            day_07::run_task_1
        } else {
            day_07::run_task_2
        }),
        "08" => Some(if task == "1" {
            day_08::run_task_1
        } else {
            day_08::run_task_2
        }),
        "10" => Some(if task == "1" {
            day_10::run_task_1
        } else {
            day_10::run_task_2
        }),
        "11" => Some(if task == "1" {
            day_11::run_task_1
        } else {
            day_11::run_task_2
        }),
        "12" => Some(if task == "1" {
            day_12::run_task_1
        } else {
            day_12::run_task_2
        }),
        "13" => Some(if task == "1" {
            day_13::run_task_1
        } else {
            day_13::run_task_2
        }),
        "14" => Some(if task == "1" {
            day_14::run_task_1
        } else {
            day_14::run_task_2
        }),
        _ => None,
    };

    let src_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let file_path = src_path.join(format!(
        "day_{}\\data\\task_{}{}.txt",
        day,
        task,
        if is_test { "_test" } else { "" }
    ));
    println!("~ Data: {:?}", file_path);
    let data = std::fs::read_to_string(&file_path).unwrap();

    match task_func {
        Some(func) => {
            measure_time!(func, data);
        }
        None => println!("Unknown task"),
    }
}

fn handle_arguments(args: &[String]) -> (String, String, bool) {
    let mut day;
    let task;
    let mut is_test = false;

    if args.len() < 3 {
        panic!("Please specify a day and task. Day 1 - Task 1 would be: `cargo run 01 1`");
    }

    // Handle `day` argument
    day = args[1].as_str().to_string();
    if let Ok(num) = day.parse::<u32>() {
        if num < 10 {
            day = format!("0{}", num); // Add leading zero
        }
    } else {
        panic!("Day must be a valid number");
    }

    // Handle `task` argument (must be 1 or 2)
    if let Ok(num) = args[2].parse::<u32>() {
        if num < 1 || num > 2 {
            panic!("Task must be 1 or 2");
        }
        task = num.to_string();
    } else {
        panic!("Task must be a valid number");
    }

    // Handle `is_test` flag (either "--test" or "-t")
    if args.len() > 3 {
        is_test = args[3] == "--test" || args[3] == "-t";
    }

    (day, task, is_test)
}
