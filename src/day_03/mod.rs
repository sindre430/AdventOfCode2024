use regex::Regex;

pub fn run_task_1(data: String) {
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut sum = 0;
    for capture in regex.captures_iter(&data) {
        let capture_value = capture.get(0).unwrap().as_str();
        let val = do_mul(&capture_value);

        println!("{} = {}", capture_value, val);
        sum += val;
    }

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {
    let regex = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();

    let mut sum = 0;
    let mut mul_enabled = true;
    for capture in regex.captures_iter(&data) {
        let capture_value = capture.get(0).unwrap().as_str();

        match capture_value {
            "do()" => {
                mul_enabled = true;
                println!("** mul_enabled = true");
            }
            "don't()" => {
                mul_enabled = false;
                println!("** mul_enabled = false");
            }
            _ => {
                if mul_enabled {
                    let val = do_mul(&capture_value);
                    println!("{} = {}", capture_value, val);
                    sum += val;
                } else {
                    println!("{} (Ignored)", capture_value);
                }
            }
        };
    }

    println!("Sum: {}", sum);
}

fn do_mul(expr: &str) -> i32 {
    let trimmed = remove_chars(&expr, 4, 1);
    let x = trimmed.split(",").collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();
    let y = trimmed.split(",").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();
    x * y
}

fn remove_chars(s: &str, num_start: usize, num_end: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if num_start + num_end >= chars.len() {
        return String::new(); // Return empty if the removal range is invalid
    }
    chars[num_start..chars.len() - num_end].iter().collect()
}
