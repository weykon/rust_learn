// 让我们先主动试试让函数作为参数
// 这里非常需要注意的是，fn(i32,i32)->i32
// 我们是在定义函数类型而不是具体的函数，
// 可能是我在ts的习惯
// function math(fn : (a,b)=>{}){}； 所以写了参数名字进去
fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
    op(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let a = 2;
    let b = 3;
    math(add, a, b);
}

