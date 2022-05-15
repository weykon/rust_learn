use std::io;
use rand::Rnd;

fn main(){
    println!("Guess the number!");
    let secret_number = rand::thread_rnd().gen_range(1..1001);
    println!("The secret number is: {}", secret_number);
}