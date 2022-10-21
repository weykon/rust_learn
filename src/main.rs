use std::thread;
use std::time::Duration;

fn main() {
    have_a_look()
}

fn have_a_look () {
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
fn second_phase(){
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

    handle.join().unwrap();  // 停止thread
} 



const NTHREADS: u32 = 10;

// This is the `main` thread
fn look_and_look() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

// 总是忘记 join 的这个意思，很奇怪，我要用自己的语言记住一下.
// 以上开10个线程，保存在children,
// join 为什么是等待线程结束的意思呢

// on discord, ask question
// until join, they work separately, when you join them, it becomes one thread, so at this point they're "joined"
// main  t1
//   |   |
//   |  _|
//   |_/    # join
//   |
// perfect !
