// 让我们先主动试试让函数作为参数
// 这里非常需要注意的是，fn(i32,i32)->i32
// 我们是在定义函数类型而不是具体的函数，
// 可能是我在ts的习惯
// function math(fn : (a,b)=>{}){}； 所以写了参数名字进去
fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
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

    ref_and_mut_and_refmut();
}


// 可变引用和不可变引用和可变变量在输入函数做参数的区别
fn ref_and_mut_and_refmut (){

    fn add_one(mut x: i32) {
        x += 1;
        println!("Inside function (mut x: i32): x = {}", x);
    }

    fn add_one_ref(x: &mut i32) {
        *x += 1;
        println!("Inside function (&mut i32): x = {}", x);
    }

    let x = 5;
    add_one(x);  // 在这里面的x输出是 6 
    println!("Outside function: x = {}", x); // 但是出了来是 5 , 是进入函数拷贝了一份x

    let mut y = 5;
    add_one_ref(&mut y);
    println!("Outside function: y = {}", y);
}
// (mut a: i32)
// 是在进入函数的时候做了一份拷贝，大概去满足与在传入的参数不对外去进行修改，但是在函数中有修改的时候 
