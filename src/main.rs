fn main() {
    let nums = 10;
    (1..nums).fold(0, |acc, x| acc + x);
}
