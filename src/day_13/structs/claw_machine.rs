pub struct ClawMachine {
    pub a_vec: (i64, i64),
    pub a_cost: i64,
    pub b_vec: (i64, i64),
    pub b_cost: i64,
    pub prize_pos: (i64, i64),
}

impl ClawMachine {
    pub fn new(
        a_vec: (i64, i64),
        a_cost: i64,
        b_vec: (i64, i64),
        b_cost: i64,
        prize_pos: (i64, i64),
    ) -> ClawMachine {
        ClawMachine {
            a_vec,
            a_cost,
            b_vec,
            b_cost,
            prize_pos,
        }
    }

    pub fn find_cheapest_win(&self) -> i64 {
        let i = (self.b_vec.1 * self.prize_pos.0 - self.b_vec.0 * self.prize_pos.1)
            / (self.a_vec.0 * self.b_vec.1 - self.a_vec.1 * self.b_vec.0);
        let j = (self.prize_pos.1 - i * self.a_vec.1) / self.b_vec.1;

        if self.a_vec.0 * i + self.b_vec.0 * j != self.prize_pos.0
            || self.a_vec.1 * i + self.b_vec.1 * j != self.prize_pos.1
        {
            return 0;
        }

        let sum = self.a_cost * i + self.b_cost * j;

        sum
    }
}
