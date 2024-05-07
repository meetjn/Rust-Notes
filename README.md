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

## Guessing Game Project

Quick set-up

$ cargo new guessing_game
$ cd guessing_game

The first command, cargo new, takes the name of the project (guessing_game) as the first argument. The second command changes to the new project’s directory.

The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.

In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.

          let mut guess = String::new();

     
Returning to the guessing game program, you now know that let mut guess will introduce a mutable variable named guess. The equal sign (=) tells Rust we want to bind something to the variable now. On the right of the equal sign is the value that guess is bound to, which is the result of calling String::new, a function that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.

In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!

     .read_line(&mut guess)

Next, the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references

## Handling Potential Failure with Result

           .expect("Failed to read line");

As mentioned earlier, read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.

Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

If you don’t call expect, the program will compile, but you’ll get a warning:

## Printing Values with println! Placeholders

This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place. When printing the value of a variable, the variable name can go inside the curly brackets. 

## Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.

Cargo’s coordination of external crates is where Cargo really shines. Before we can write code that uses rand, we need to modify the Cargo.toml file to include the rand crate as a dependency. Open that file now and add the following line to the bottom, beneath the [dependencies] section header that Cargo created for you.


When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the [dependencies] section and downloads any crates listed that aren’t already downloaded. In this case, although we only listed rand as a dependency, Cargo also grabbed other crates that rand depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

rand::thread_rng function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system. Then we call the gen_range method on the random number generator. This method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement. The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.

## Allowing Multiple Guesses with Looping

The loop keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number:     

## Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a non-number so the user can continue guessing. We can do that by altering the line where guess is converted from a String to a u32


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resultant number. That Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value. That number will end up right where we want it in the new guess variable we’re creating.

If parse is not able to turn the string into a number, it will return an Err value that contains more information about the error. The Err value does not match the Ok(num) pattern in the first match arm, but it does match the Err(_) pattern in the second arm. The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code, continue, which tells the program to go to the next iteration of the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!