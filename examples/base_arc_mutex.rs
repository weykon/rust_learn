use std::{thread, sync::{Arc, Mutex}};
use colored::Colorize;
use num_cpus;

/// 这里都是错误的代码,先去好好写写arc和mutex
/// 
fn main() {
    println!(
        "Hello, This is the {} .",
        "BASE Arc Mutex Playground".cyan().bold()
    );

    let cpus = num_cpus::get();
    let mut handles = Vec::new();

    let mut tasks =Arc::new(Mutex::new( Vec::new()));
    tasks.push(Arc::new(Mutex::new(1)));
    tasks.push(Arc::new(Mutex::new(2)));

    (0..cpus).for_each(|id| {
        handles.push(id);
        let id = thread::spawn(move || {
            println!("thread id: {}", id);

            // do something
            if let Some( task) = tasks.pop() {
                let mut task = task.lock().unwrap();
                *task += 1;
                println!("task: {}", task);
            }


        });
        id.join().unwrap();
    });

    println!("all threads are finished!");
}
