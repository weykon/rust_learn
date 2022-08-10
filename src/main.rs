use std::thread;
use std::time::Duration;
mod thread_move;
mod event_begin_chennel;

fn main() {
    have_a_look();
    thread_move::main();
    event_begin_chennel::main();
}

fn have_a_look() {
    thread::spawn(|| {
        for i in 1..3 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..8 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// join
// 调用join句柄会阻塞当前运行的线程
fn second_phase() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); 
    // 这条命令是要求主程序等待这个线程完成才完成，（允许线程继续跑下来的意思）
    // 没有这条的话，主程序是不理线程的运行情况，主程序自己跑完，就关闭了，但是
    // 这个线程如果没跑完的话，就一辈子跑不完。
}
