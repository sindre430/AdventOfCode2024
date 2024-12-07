mod structs;
use std::sync::mpsc;
use std::thread;

use structs::equation::Equation;

pub fn run_task_1(data: String) {
    let equations = parse_input(data);

    let (tx, rx) = mpsc::channel();
    for equation in equations.into_iter() {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(if equation.check_equation(false) {
                equation.sum
            } else {
                0
            })
            .unwrap();
        });
    }

    drop(tx);

    let mut sum = 0;
    for eq_sum in rx {
        sum += eq_sum;
    }

    println!("Sum: {}", sum);
}

pub fn run_task_2(data: String) {
    let equations = parse_input(data);

    let (tx, rx) = mpsc::channel();
    for equation in equations.into_iter() {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(if equation.check_equation(true) {
                equation.sum
            } else {
                0
            })
            .unwrap();
        });
    }

    drop(tx);

    let mut sum = 0;
    for eq_sum in rx {
        sum += eq_sum;
    }

    println!("Sum: {}", sum);
}

fn parse_input(data: String) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in data.lines() {
        let parts1 = line.split(": ").collect::<Vec<&str>>();
        let parts2 = parts1[1].split(" ").collect::<Vec<&str>>();

        equations.push(Equation::new(
            parts1[0].parse::<i64>().unwrap(),
            parts2
                .iter()
                .map(|val| val.parse().unwrap())
                .collect::<Vec<i64>>(),
        ));
    }

    equations
}
