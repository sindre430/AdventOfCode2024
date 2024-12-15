use structs::claw_machine::ClawMachine;
mod structs;

pub fn run_task_1(data: String) {
    let claw_machines = parse_input(data, (0, 0));

    let mut sum = 0;
    for claw_machine in claw_machines {
        sum += claw_machine.find_cheapest_win();
    }

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {
    let claw_machines = parse_input(data, (10000000000000, 10000000000000));

    let mut sum = 0;
    for claw_machine in claw_machines {
        sum += claw_machine.find_cheapest_win();
    }

    println!("Sum: {}", sum);
}

fn parse_input(data: String, prize_correction: (i64, i64)) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();
    let mut a_vec: (i64, i64) = (0, 0);
    let mut b_vec: (i64, i64) = (0, 0);

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        let parts1 = line.split(": ").collect::<Vec<&str>>();
        let parts2 = parts1[1].split(", ").collect::<Vec<&str>>();
        let x = parts2[0][2..].parse::<i64>().unwrap();
        let y = parts2[1][2..].parse::<i64>().unwrap();

        if line.starts_with("Button A:") {
            a_vec = (x, y);
        } else if line.starts_with("Button B:") {
            b_vec = (x, y);
        } else if line.starts_with("Prize:") {
            let claw_machine = ClawMachine::new(
                a_vec,
                3,
                b_vec,
                1,
                (prize_correction.0 + x, prize_correction.1 + y),
            );
            claw_machines.push(claw_machine);
        }
    }

    claw_machines
}
