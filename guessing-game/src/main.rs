use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let min = 1;
    let max = 100;
    let mut chance_to_play = 3;
    println!("Guess the number from {min} to {max}!");

    let secret_number = rand::thread_rng().gen_range(min..=max);
    while chance_to_play > 0 {
        println!("Please input your guess, your remaining chances is {chance_to_play}");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        chance_to_play -= 1;
    }

    if chance_to_play == 0 {
        println!("You lose!");
    }
}
