
// 方法中使用泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 前面的impl<T>是依然需要提前声明，给后面的 Point<T> 知道，它是泛型，而不是具体的类型T。
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32>{ // 这里就只有当f32的类型时候才匹配到 distance_from_origin impl
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {}
