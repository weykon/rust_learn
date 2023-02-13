// 这种场景通常发生在您需要创建动态数据结构，例如链表、树或图形，其生存周期受其他变量的生存周期影响。
// 例如，假设您需要创建一个链表，并且链表中的每个节点都需要持有一个整数值。您可以使用 Box 来动态分配内存并在链表中传递所有权：

use std::boxed::Box;
enum List {
    Cons(i32, Box<List>),
    Nil,
}
pub fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    Box_continue();
}

// 在这种情况下，每个链表节点都被分配到动态内存中，
// 并且每个节点的生存周期都受到其他节点的生存周期影响。
// 因此，使用 Box 可以更好地管理内存分配，以及链表的生存周期。

// 这里包括了enum和struct的不同
// enum很明显在分类上的应用
// struct是在于实例下的属性

#[derive(Debug)]
struct Game {
    start: String,
}

fn Box_continue() {
    // 我先不使用链表的例子，那么我设定一个未定量为一枚举
    let game = Box::new(GameState::initialized(Game {
        start: String::from("start game ..."),
    }));

    let un_game = Box::new(GameState::uninitialized);

    check_game(&game);
    check_game(&un_game);
}
#[derive(Debug)]
enum GameState {
    initialized(Game),
    uninitialized,
}

fn check_game(game: &Box<GameState>) {
    println!("what game state now : {:?}", game);
}


// Box 最重要的一个点是“创建动态数据结构”