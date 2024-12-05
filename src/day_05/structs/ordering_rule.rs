#[derive(Clone)]
pub struct OrderingRule {
    pub left: i32,
    pub right: i32,
}

impl OrderingRule {
    pub fn new(left: i32, right: i32) -> OrderingRule {
        OrderingRule { left, right }
    }
}
