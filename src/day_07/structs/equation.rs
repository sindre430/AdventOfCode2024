pub struct Equation {
    pub sum: i64,
    pub numbers: Vec<i64>,
}

impl Equation {
    pub fn new(sum: i64, numbers: Vec<i64>) -> Equation {
        Equation { sum, numbers }
    }

    pub fn check_equation(&self, use_concat: bool) -> bool {
        let mut results = Vec::new();
        Self::generate_expressions(
            &self.numbers[1..],
            self.numbers[0].to_string(),
            self.numbers[0],
            use_concat,
            &mut results,
        );

        let valid = results
            .into_iter()
            .map(|res| {
                let parts = res.split(" = ").collect::<Vec<&str>>();
                let result = parts[1].parse::<i64>().unwrap();
                result == self.sum
            })
            .any(|x| x);

        valid
    }

    fn generate_expressions(
        remaining: &[i64],
        current_expr: String,
        current_value: i64,
        use_concat: bool,
        results: &mut Vec<String>,
    ) {
        if remaining.is_empty() {
            results.push(format!("{} = {}", current_expr, current_value));
            return;
        }

        let next_number = remaining[0];
        let rest = &remaining[1..];

        // Add operation
        Self::generate_expressions(
            rest,
            format!("{} + {}", current_expr, next_number),
            current_value + next_number,
            use_concat,
            results,
        );

        // Multiply operation
        Self::generate_expressions(
            rest,
            format!("{} * {}", current_expr, next_number),
            current_value * next_number,
            use_concat,
            results,
        );

        // Concatenate operation
        if use_concat {
            Self::generate_expressions(
                rest,
                format!("{} || {}", current_expr, next_number),
                (current_value.to_string() + next_number.to_string().as_str())
                    .parse()
                    .unwrap(),
                use_concat,
                results,
            );
        }
    }
}
