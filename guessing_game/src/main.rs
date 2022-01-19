use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game !!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret_number is {}", secret_number);

    loop {
        println!("Please enter your input");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You have guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!!".red()),
            Ordering::Greater => println!("{}", "Too Big !!".red()),
            Ordering::Equal => {
                println!("{}", "You Win !!".green());
                break;
            }
        }
    }
}
