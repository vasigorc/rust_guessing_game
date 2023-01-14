use rand::Rng;
use std::cmp::Ordering;
use std::io;
use guessing_game::model::Guess;

fn main() {
    println!("Guess the number!");

    // local to the current thread of execution
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            // & - means we're passing a reference
            // (c) references are immutable by default hence we need &mut
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}.", guess.value());
    
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
