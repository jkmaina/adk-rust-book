use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Part 1: Using match to handle Result
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Opened file successfully using match.");

    // Part 2: Use a helper that propagates errors with `?`
    match read_username_from_file() {
        Ok(username) => println!("Username from file: {}", username),
        Err(e) => println!("Failed to read username: {}", e),
    }

    Ok(())
}

// The ? Operator (Propagating Errors)
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
