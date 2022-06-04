use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    fn defalut_impl(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notifyTwo(item: &impl Summary, item2: &impl Summary) {}
pub fn notifyPureInGeneric<T: Summary>(item: &T) {}
pub fn notifyWithGenericForTwoSameT<T: Summary>(item: &T, item2: &T) {}

// 多重约束
pub fn notifyMutil(item: &(impl Summary + Display)) {}
pub fn notifyMutilT<T: Summary + Display>(item: &T) {}

// Where
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
// change
fn some_function_where<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn main() {}
