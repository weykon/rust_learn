use std::collections::VecDeque;

fn main() {
    vecDeque_vec_double_ended_queue();
    binary_tree_map()
}

fn vecDeque_vec_double_ended_queue() {
    println!("vecDeque(vec_double_ended_queue)");
    let mut v1 = VecDeque::new();
    v1.push_back(1);
    v1.push_back(2);
    println!("One: {:?}", v1.pop_front());
    println!("Two: {:?}", v1.pop_front());
    v1.push_front(3);
    v1.push_front(4);
    println!("Three: {:?}", v1.pop_back());
    println!("Four: {:?}", v1.pop_back());
}

fn binary_tree_map () { 
    println!("binary_tree_map");
    let mut map = std::collections::BTreeMap::new();
    map.insert(3, "c");
    map.insert(2, "b");
    map.insert(1, "a");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}