// include the input/output functionality from the standard library 
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        // let creates variable, immutable by default.
        // mut makes variable mutable.
        // :: ~ static method, from the Type not an instance.
        // creates a String by type inference.
        let mut guess = String::new();

        // stdin function returns an instance of std::io::Stdin .
        // read_line take whatever the user types into standard input
        //    and place that into a string.
        // & indicates that the argument is a reference.
        // read_line returns a Result type.
        // io::Result has an expect method.
        // if Result is Err, program crashes and display the message.
        // if Result is Ok, expect returns Ok.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // allows us to shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name.
        // colon (:) after guess tells to annotate the variable’s type.
        // parses a string into some kind of number.
        // Switching from an expect call to a match expression is how you
        //   generally move from crashing on an error to handling the error. 
        //  underscore, _, is a catchall value.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} are placeholders to print variable values
        println!("You guessed: {}", guess);

        // pattern matching
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
