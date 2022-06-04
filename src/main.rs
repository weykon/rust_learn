use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
} // 以上这个方法，只有在T的类型满足Display+PartialOrd的类型下才能使用cmp_display的方法，
  // 这个特性就十分吸引我了，
  //因为在ts里一直以来我都想拥有这个能自适应下的类型配对的书写过程实现

fn main() {}
