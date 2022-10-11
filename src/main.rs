// :: slice::windows

fn main() {
    let slice = ['f', 'o', 'o'];
    let mut iter = slice.windows(1);
    assert!(iter.next().is_none());
}
