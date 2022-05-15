use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let mut rng = thread_rng();

    let secret_number: u32 = rng.gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 这条如果放在loop外面就不能正常运行了

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
