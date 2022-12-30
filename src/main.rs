trait Behavior {
    fn eat(&self) {
        println!("真香")
    }
    fn make_sound(&self);
}
struct Dog;
struct Cat;
impl Behavior for Dog {
    fn make_sound(&self) {
        println!("wang!")
    }
}
impl Behavior for Cat {
    fn make_sound(&self) {
        println!("miao!")
    }
}

// 下面的是语法糖
fn feed(item: impl Behavior) {
    item.eat();
}

fn real_format_feed<T: Behavior>(item: T) {
    item.eat();
}

fn main() {
    let dog = Dog {};
    feed(dog);
}
