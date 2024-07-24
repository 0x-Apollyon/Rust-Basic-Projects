extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number, it is between 1-10");

    let secret_number = rand::thread_rng().gen_range(1, 11);
    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess_int = guess.trim().parse::<i32>().unwrap();

    if secret_number == guess_int{
        println!("Your guess is corret. The number is indeed {}" , secret_number);
    }
    else{
        println!("Your guess is incorrect. You guessed {} and the number was {}" , guess_int , secret_number);
    }   
}