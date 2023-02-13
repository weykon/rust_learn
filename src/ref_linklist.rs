// 那引用的话，为什么不能实现呢？

// 引用本身不能保存数据，而是引用其他已经存在的数据。
// 在创建链表时，每个节点都应该保存它自己的数据，同时也需要引用下一个节点。
// 因此，如果使用引用，很难指向下一个节点，因为该引用可能不存在，直到下一个节点被创建。

// struct Node {
//     data: i32,
//     next: Option<&Node>,    // consider introducing a named lifetime parameter: `<'a>`, `'a `
// }

// impl Node {
//     fn new(data: i32) -> Node {
//         Node {
//             data: data,
//             next: None,
//         }
//     }
// }
// fn main() {
//     let node1 = Node::new(1);
//     let node2 = Node::new(2);

//     // 这里不能使用引用，因为它指向一个可能不存在的对象
//     node1.next = Some(&node2);
// }


// ？ ！为什么可能不存在呢，好，先不回答，讨论如果使用引用来指引node2，那么就必须node2这个数据一直在内存中，
// 引用能一直有效，但是我们链表在应用中常常会删除，删除了的数据难道一直用一个空值来替代原来的位置吗
// 所以有了动态增删的内容，就使用Box来处理，直接引用不是一个最优解。


// 悬垂引用警告和提示
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }