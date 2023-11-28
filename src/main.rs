use std::collections::VecDeque;

fn main() {
    vecDeque_vec_double_ended_queue();
    binary_tree_map();
    let a_string_vec = to_upper_case(&["hello", "world"]);
    println!("{:?}", a_string_vec);
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

fn binary_tree_map() {
    println!("binary_tree_map");
    let mut map = std::collections::BTreeMap::new();
    map.insert(3, "c");
    map.insert(2, "b");
    map.insert(1, "a");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

fn iter_or_into_iter() {
    println!("iter_or_into_iter");
    // 这里说明了，iter() 和 into_iter() 的区别
    // iter是拿到引用，into_iter是拿到所有权
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let mut into_iter = v2.into_iter();

    into_iter.for_each(|mut x| x *= 2);
}

// 写一个函数，接收一个字符串切片数组 &[&str]，
// 使用 iter 和链式调用将每个元素转换为其大写形式，
// 并收集到一个新的 Vec<String> 中返回。
fn to_upper_case(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|x| x.to_uppercase()).collect()
}

// 给定一个整数数组 vec![1, 2, 3, 4, 5]，写两个函数：
// 第一个函数使用 iter 来遍历数组并返回每个元素的平方。
// 第二个函数使用 into_iter 来遍历数组，并返回一个新数组，其中包含原数组每个元素的平方。
fn square(vec: Vec<i32>) -> Vec<i32> {
    vec.iter().map(|x| x * x).collect()
} // 不可变引用下的，读取他的数据后，map到一个每个新的x*x的数据放到一个collect 的意思 
fn square2(vec: Vec<i32>) -> Vec<i32> {
    vec.into_iter().map(|x| x * x).collect()
} // 拿原来本身，因为直接占用的所有权，
// 上面这两个，第一个是拿到引用，第二个是拿到所有权

// 对于iter下
// 迭代器中的“副本”
// 在 vec.iter().map(|x| x * x) 这个链式调用中，iter 方法创建了一个迭代器，它产生了对 vec 中每个元素的不可变引用。这里的关键点是：
// 不是物理副本：这些引用指向原始 vec 中的数据，而不是它们的物理副本。没有新的内存分配来存储数据的副本。
// 逻辑副本：当我们在 map 中使用 |x| x * x，每次迭代传递给闭包的 x 是对原始元素的引用的解引用（dereference）。这意味着 x 本身是原始数据的一个值拷贝，但这种拷贝是逻辑上的，因为它发生在寄存器或堆栈级别，而不是在内存中创建了一个新的、独立的数据实例。

// 在 vec.iter().map(|x| x * x) 这个调用中，
// 每次迭代操作实际上是在原始数据的一个简单值拷贝上进行的，
// 但这并不涉及到整个 vec 或其元素的深度拷贝（deep copy）