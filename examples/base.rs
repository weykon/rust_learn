use colored::Colorize;
use num_cpus;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("Hello, This is the {} .", "BASE Playground".cyan().bold());

    let a_thing = Arc::new(1);
    let b_thing = Arc::new(2);

    let c_thing = Mutex::new(3);
    let d_thing = Mutex::new(4);
    println!(" the first time check the count ,{}" , Arc::strong_count(&a_thing));
    let for_a_to_other_thread_so_need_clone_anothor = Arc::clone(&a_thing);
    let handle = thread::spawn(move || {
        println!(
            "thread: inner : a {} ,  ",
            for_a_to_other_thread_so_need_clone_anothor
        );
        println!(" we can check the count ,{}" , Arc::strong_count(&for_a_to_other_thread_so_need_clone_anothor));
    });
    handle.join().unwrap();

    // 再次复习
    // Arc，或者 "Atomic Reference Counting"，是一种线程安全的引用计数类型。
    // 它允许多个线程拥有某个值的引用，并在最后一个引用消失时自动清理该值。

    // Mutex，或者 "Mutual Exclusion"，是一种同步原语，用于在多线程环境中保护对数据的访问。
    // 当一个线程拥有一个 Mutex，其他线程就不能访问被 Mutex 所保护的数据，除非拥有 Mutex 的线程释放了它。

    println!("a {} , b {} ", a_thing, b_thing);
    println!("c {:?} , d {:?} ", c_thing, d_thing);
    let mut c_thing_guard = c_thing.lock().unwrap();
    *c_thing_guard += 1;
    println!("c {:?} , d {:?} ", c_thing, d_thing);
}
