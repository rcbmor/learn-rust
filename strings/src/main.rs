
#![allow(unused_variables)]

fn slices() {

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("hello str {}", hello);
    println!("slice [0..4] {}", s);

}

fn utf8str() {
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn examplestr() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();


    let s1 = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);
    let s3 = s1 + &s2;

}

fn main() {
    examplestr();
    utf8str();
    slices();
}
