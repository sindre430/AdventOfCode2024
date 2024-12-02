pub struct Report {
    pub levels: Vec<i32>,
}

impl Report {
    pub fn new(numbers: Vec<i32>) -> Self {
        Report { levels: numbers }
    }

    pub fn check_if_safe(&self, max_diff: u32, error_tolerance: u32) -> bool {
        Self::validate_levels(&self.levels, max_diff, error_tolerance)
            || Self::validate_levels(
                &self.levels.iter().rev().cloned().collect(),
                max_diff,
                error_tolerance,
            )
    }

    fn validate_levels(levels: &Vec<i32>, max_diff: u32, error_tolerance: u32) -> bool {
        let total_windows = levels.windows(2).count();
        let increasing_count = levels.windows(2).filter(|w| w[0] < w[1]).count();
        let is_increasing = increasing_count > total_windows / 2;
        let mut prev_level: Option<i32> = None;
        let mut error_count = 0;

        for &level in levels {
            match prev_level {
                Some(prev_level_val) => {
                    let diff = level - prev_level_val;
                    if is_increasing && diff < 0 || !is_increasing && diff > 0 || diff == 0 {
                        error_count += 1;
                    } else if diff.unsigned_abs() > max_diff {
                        error_count += 1;
                    } else {
                        prev_level = Some(level);
                    }

                    if error_count > error_tolerance {
                        return false;
                    }
                }
                None => {
                    prev_level = Some(level);
                }
            }
        }

        true
    }

    pub fn levels_to_string(&self) -> String {
        String::from(
            self.levels
                .iter()
                .map(|level| level.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
