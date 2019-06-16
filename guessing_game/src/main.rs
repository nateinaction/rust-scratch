extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number, it's between 1 and 100!");

    loop {
        // vars are immutable by default, we must declare mutability
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // How to annotate type -> let name: type = value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

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
