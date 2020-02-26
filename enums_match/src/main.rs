
#![allow(unused_variables)]
fn enums_1() {

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // Matches in Rust are exhaustive: we must exhaust every last possibility 
        match coin {
            Coin::Penny => {
                println!("Penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25},
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn enum_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),  // pattern will match any value.
    }
}

fn if_let_ex() {
#[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin1 = Coin::Penny;
    let mut count1 = 0;
    match coin1 {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count1 += 1,
    }


    let coin2 = Coin::Penny;
    let mut count2 = 0;
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count2 += 1;
    }
}


fn main() {
    enums_1();
    enum_option();
    placeholder();
}
