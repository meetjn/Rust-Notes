use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();   // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to  read line");

    println!("You guessed: {guess}");
}