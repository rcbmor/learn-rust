
use std::fs::File;
use std::io::ErrorKind;

fn panic_vector() {
    let v = vec![1, 2, 3];
    v[99];
}

fn result_test() {
    let f = File::open("hello.txt");
}

fn result_test_fail() {
   // let f: u32 = File::open("hello.txt");
   //       |   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
}

fn result_match() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:22:23
}

fn result_match_error() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn result_no_match() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn result_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn result_expect() {
    let f = File::open("hello.txt").expect("(My own message for panic) Failed to open file.");
}

fn main() {
   result_expect();
}
