struct Bar;

trait Foo1 {
    fn foo(&self) {}
}
trait Foo2 {
    fn foo(&self) {}
}
impl Foo1 for Bar {}
impl Foo2 for Bar {}

fn main() {
    let a = Bar;
    a.foo()
}

// help: disambiguate the associated function for candidate #1
//    |
// 16 |     Foo1::foo(&a)
//    |