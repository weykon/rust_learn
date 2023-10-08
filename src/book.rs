/// 这里是看书《Rust编程之道》的内容
///
/// 泛型trait example
///

trait Add<RHS, Output> {
    fn add(self, rhs: RHS) -> Output;
}

impl Add<i32, i32> for i32 {
    fn add(self, rhs: i32) -> i32 {
        self + rhs
    }
    fn my_add (self, rhs: i32) -> i32 {
        self + rhs
    }
}
