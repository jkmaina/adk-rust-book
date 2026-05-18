use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let sample_file = sample_file_path();
    ensure_sample_file(&sample_file)?;

    // Part 1: Using match to handle Result
    let f = File::open(&sample_file);

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("Opened file successfully using match.");

    // Part 2: Use a helper that propagates errors with `?`
    match read_username_from_file(&sample_file) {
        Ok(username) => println!("Username from file: {}", username),
        Err(e) => println!("Failed to read username: {}", e),
    }

    Ok(())
}

fn sample_file_path() -> PathBuf {
    std::env::temp_dir().join("adk-rust-book-result-example-hello.txt")
}

fn ensure_sample_file(path: &PathBuf) -> io::Result<()> {
    if fs::metadata(path).is_err() {
        fs::write(path, "alice\n")?;
        println!("Created sample hello.txt for the error-handling demo.");
    }

    Ok(())
}

// The ? Operator (Propagating Errors)
fn read_username_from_file(path: &PathBuf) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}
