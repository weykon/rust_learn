// 在算法题中，遇到link list的操作时
// 在对于 tree node 的操作中有一步骤: borrow_mut()

// me:
// some had signed mutable reference, can be borrow to
// this trait which signed.

// Mutably borrows from an owned value.
use std::borrow::BorrowMut;

fn check<T: BorrowMut<[i32]>>(mut v: T) {
    assert_eq!(&mut [1, 2, 3], v.borrow_mut());
}
pub fn main() {
    let v = vec![1, 2, 3];
    check(v);
}
