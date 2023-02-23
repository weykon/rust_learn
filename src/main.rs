fn main() {
    let a = vec![1, 2, 3];

    let b = vec![3, 2, 1];

    for (a, b) in a.iter().zip(b.iter()) {
        println!("{},{}", a, b);
    }

    // 1,3
    // 2,2
    // 3,1
}
