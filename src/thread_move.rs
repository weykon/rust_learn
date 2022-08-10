use std::thread;
use std::time::Duration;
pub fn main(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        // println!("Here's a vector: {:?}", v);  这里会错误，因为主程的v变量怎么能在线程里跑
    });


    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);  // 这里使用move 强制线程获取使用的值的所有权。
    });

    handle.join().unwrap();

    // println!("can i borrow the v ? {:?}" ,v);  // No
}