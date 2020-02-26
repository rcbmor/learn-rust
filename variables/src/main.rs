
#![allow(unused_variables)]

fn compound_types_arrays() {
    // a collection of multiple values is with an array
    // must have the same type.
    let a = [1, 2, 3, 4, 5];
    // typed array of i32 with 5 elements
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let five_threes = [3; 5];
    // accessing array elements
    let first = a[0];
    let second = a[1];
    // result in a runtime error:
    // let index_out_of_bounds = a[10]

}

fn compound_types_tuples() {
    // TUPLES
    // general way of grouping together a number of values with a variety of types into one compound type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring through pattern matching, breaks the single tuple into three parts
    let (x, y, z) = tup;
    println!("Tuple, value of y is: {}", y);
    // access a tuple element directly
    let t: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple, value of t.0: {}", t.0);
}

fn character_type() {
// Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Characters: {} {} {}", c, z, heart_eyed_cat);
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

}

fn imutable() {
    // fix by adding mut modifier
    let mut x = 5;
    //let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
//  ^^^^^ cannot assign twice to immutable variable
//  When a variable is immutable, once a value is bound to a name, you can’t change that value.
//
    println!("The value of x is: {}", x);
}

fn main() {
    // underscores can be inserted in numeric literals to improve readability
    const MAX_POINTS: u32 = 100_000;
    imutable();
    character_type();
    compound_types();
}

