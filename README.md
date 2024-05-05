# Rust-Notes

Getting started

Writing a program that prints Hello, world!
Using cargo, Rust’s package manager and build system

### Run this command to install rust if you are on mac 

     curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
Check if the rust is intalled correctly by this command

      rustc --version

Writing first program Hello, World!

Create a new file with a name main.rs , the rust extension is .rs and type the code below

fn main() {
    println!("Hello, world!");
}

Save the program and run the command

    $ rustc main.rs
    $ ./main

you should see the below output. congrats!
Hello, world!

## Anatomy of a Rust Program

fn main() {

}

These lines define a function named main. The main function is special: it is always the first code that runs in every executable Rust program. Here, the first line declares a function named main that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses ().

The function body is wrapped in {}. Rust requires curly brackets around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

The body of the main function holds the following code:

println!("Hello, world!");
