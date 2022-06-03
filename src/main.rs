fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

fn main() {
    println!("add i18:{}", add_i8(2i8, 3i8));
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
