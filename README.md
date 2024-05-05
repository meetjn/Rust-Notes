# Rust-Notes

Getting started

Writing a program that prints Hello, world!

Using cargo, Rust’s package manager and build system

### Run this command to install Rust if you are on a mac 

     curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
Check if the rust is installed correctly by this command

      rustc --version

Writing the first program Hello, World!

Create a new file with the name main.rs , the rust extension is .rs and type the code below

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

The function body is wrapped in {}. Rust requires curly brackets around all function bodies. It’s a good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

The body of the main function holds the following code:

println!("Hello, world!");

This line does all the work in this little program: it prints text to the screen. There are four important details to notice here.

First, Rust style is to indent with four spaces, not a tab.

Second, println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

Third, you see the "Hello, world!" string. We pass this string as an argument to println!, and the string is printed to the screen.

Fourth, we end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.

## Cargo in Rust

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

The simplest Rust programs, like the one we’ve written so far, don’t have any dependencies. If we had built the “Hello, world!” project with Cargo, it would only use the part of Cargo that handles building your code. As you write more complex Rust programs, you’ll add dependencies, and if you start a project using Cargo, adding dependencies will be much easier to do.

Creating a Project with Cargo

Let’s create a new project using Cargo and look at how it differs from our original “Hello, world!” project. Navigate back to your projects directory (or wherever you decided to store your code). Then, on any operating system, run the following:

$ cargo new hello_cargo
$ cd hello_cargo

Cargo has generated a “Hello, world!” program for you, So far, the differences between our project and the project Cargo generated are that Cargo placed the code in the src directory and we have a Cargo.toml configuration file in the top directory.

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the src directory and create an appropriate Cargo.toml file.

## Building and Running a Cargo Project

Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your hello_cargo directory, build your project by entering the following command:

     $ cargo build

This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug.

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command:

     $ cargo run

Using cargo run is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use cargo run.

Notice that this time we didn’t see output indicating that Cargo was compiling hello_cargo. Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it, and you would have seen this output:

Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

     cargo check

Why would you not want an executable? 

Often, cargo check is much faster than cargo build because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using cargo check will speed up the process of letting you know if your project is still compiling! As such, many Rustaceans run cargo check periodically as they write their program to make sure it compiles. Then they run cargo build when they’re ready to use the executable.

## Building for Release

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.
