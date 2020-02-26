fn main() {
    another_function(5, 6);
    statements_vs_expressions();

    let x = five();
    println!("The value of x is: {}", x);

    conditionals();
    if_is_an_expression();
    return_value_from_loop();
    loop_on_collection();
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resulting value
// If you add a semicolon to the end of an expression, you turn it into a statement
fn statements_vs_expressions() {
   let x = 5; // statement

    let y = {
        let x = 3; // statement
        x + 1  // expression, returns x + 1
    };

    println!("The value of y is: {}", y);

}

// functions with return values

// You can return early from a function by using the return keyword and specifying a value,
// but most functions return the last expression implicitly. 
fn five() -> i32 {
    5
}

fn conditionals() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_is_an_expression() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn loop_on_collection() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

}
