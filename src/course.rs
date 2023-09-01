struct Test {
    a : u32,
    b : u32
}

impl Test {
    fn increase(&mut self) {
        let mut a = &mut self.a;
        let mut b = &mut self.b;
        *b += 1;
        *a += 1;
    }
}


pub fn main() {
    let mut t = Test { a: 1, b: 2 };
    t.increase();
    println!("a: {}, b: {}", t.a, t.b);
}