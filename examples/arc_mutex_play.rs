use colored::Colorize;
use num_cpus;
use std::{fs, thread};

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

fn main() {
    println!(
        "Hello, This is the {} .",
        "Arc Mutex Playground".cyan().bold()
    );

    let mainT = MainTerminal { task: Vec::new() };

    let cpus = num_cpus::get();
    println!("cpus: {}", cpus);
    /// handles 这里就像是如果有一个任务持有了，lock中，它在thread直到这个任务完成释放
    let handles = Vec::new();
    (0..cpus).for_each(|| {
        let id = thread::spawn(move || {
           
        });

        handles.push(id);
    })
}

struct MainTerminal {
    task: Vec<Arc<Mutex<Task>>>,
}

enum Task {
    ReadFile(PathBuf),
    SearchDirectory(PathBuf),
}

/// 目前这里是单一线程的代码，
/// 在上层的代码中，需要在每个线程中都有一个worker去拿main的tasks
fn worker(tasks: Arc<Mutex<Vec<Task>>>) {
    loop {
        let task = {
            let mut tasks = tasks.lock().unwrap();
            tasks.pop()
        };

        match task {
            Some(Task::ReadFile(path)) => {
                // read and process the file
                println!("ReadFile: {}", path.display());
            }
            Some(Task::SearchDirectory(path)) => {
                if let Ok(entries) = fs::read_dir(path) {
                    let mut tasks = tasks.lock().unwrap();
                    for entry in entries {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_dir() {
                                    tasks.push(Task::SearchDirectory(path));
                                } else if path.is_file() {
                                    tasks.push(Task::ReadFile(path));
                                }
                            }
                            Err(_) => {
                                // handle error
                            }
                        }
                    }
                }
            }
            None => {
                // no more tasks, exit the loop
                break;
            }
        }
    }
}
