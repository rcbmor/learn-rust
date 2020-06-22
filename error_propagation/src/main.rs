use std::fs;
use std::io;
use std::fs::File;
use std::io::Read;


fn read_username_from_file_longversion() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_questionoperator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // if Err returns Err out of function,
                               // implicity type conversion to function return type Err
                               // relates to "from" trait for type convertion
                               // if Ok, return ok to expression and move on
    Ok(s)
}

fn read_username_from_file_shortversion() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let s = read_username_from_file_shortest();
}
