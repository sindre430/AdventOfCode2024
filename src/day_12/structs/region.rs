use std::collections::HashMap;

use super::plot::Plot;

pub struct Region {
    pub plots: Vec<Plot>,
}

impl Region {
    pub fn new() -> Region {
        Region { plots: Vec::new() }
    }

    pub fn get_price(&self) -> i32 {
        let perimeter = self
            .plots
            .iter()
            .map(|p| 4 - p.neighbors.len() as i32)
            .sum::<i32>();
        self.plots.len() as i32 * perimeter
    }

    pub fn get_bulk_price(&self) -> i32 {
        let num_sides = self.get_num_sides();

        self.plots.len() as i32 * num_sides
    }

    fn get_num_sides(&self) -> i32 {
        let mut vertical_left_sides = HashMap::<i32, Vec<i32>>::new();
        let mut vertical_right_sides = HashMap::<i32, Vec<i32>>::new();
        let mut horizontal_top_sides = HashMap::<i32, Vec<i32>>::new();
        let mut horizontal_bottom_sides = HashMap::<i32, Vec<i32>>::new();

        let insert_side = |map: &mut HashMap<i32, Vec<i32>>, key: i32, value: i32| {
            map.entry(key).or_insert_with(Vec::new).push(value);
        };

        for plot in &self.plots {
            if !plot.neighbors.contains(&(plot.x - 1, plot.y)) {
                insert_side(&mut vertical_left_sides, plot.x, plot.y);
            }

            if !plot.neighbors.contains(&(plot.x + 1, plot.y)) {
                insert_side(&mut vertical_right_sides, plot.x + 1, plot.y);
            }

            if !plot.neighbors.contains(&(plot.x, plot.y - 1)) {
                insert_side(&mut horizontal_top_sides, plot.y, plot.x);
            }

            if !plot.neighbors.contains(&(plot.x, plot.y + 1)) {
                insert_side(&mut horizontal_bottom_sides, plot.y + 1, plot.x);
            }
        }

        let count_sides = |sides_map: HashMap<i32, Vec<i32>>| -> i32 {
            sides_map
                .into_values()
                .map(|mut sides| {
                    sides.sort();
                    let mut count = 1;
                    for pair in sides.windows(2) {
                        if pair[1] != pair[0] + 1 {
                            count += 1;
                        }
                    }
                    count
                })
                .sum::<i32>()
        };

        let num_vertical_sides =
            count_sides(vertical_left_sides) + count_sides(vertical_right_sides);
        let num_horizontal_sides =
            count_sides(horizontal_top_sides) + count_sides(horizontal_bottom_sides);

        num_vertical_sides + num_horizontal_sides
    }
}
