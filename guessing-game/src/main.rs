use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    loop {
        println!("-------NEW GAME-------");
        println!("Guess the Number!");

        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Please try again with a valid number input"); continue } ,
        };

        println!("You guessed {}", guess);
        println!("The secret number was: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
