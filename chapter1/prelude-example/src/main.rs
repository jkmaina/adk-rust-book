// The Rust Prelude - automatically imported without explicit use statements
// https://doc.rust-lang.org/std/prelude/

fn main() {
    println!("=== Rust Prelude Examples ===\n");

    // Primitive types (available from prelude)
    let _i: i32 = 42;
    let _f: f64 = 3.14;
    let _b: bool = true;
    let _c: char = 'A';
    let _s: &str = "hello";
    println!("✓ Primitive types (i32, f64, bool, char, &str) are from prelude");

    // Vec - dynamic array
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("✓ Vec: {:?}", v);

    // String - heap-allocated string
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("✓ String: {}", s);

    // Option enum (part of prelude)
    let some_value: Option<i32> = Some(5);
    let _none_value: Option<i32> = None;
    match some_value {
        Some(x) => println!("✓ Option::Some({})", x),
        None => println!("None"),
    }

    // Result enum (part of prelude)
    let ok_value: Result<i32, String> = Ok(42);
    let _err_value: Result<i32, String> = Err("error".to_string());
    match ok_value {
        Ok(x) => println!("✓ Result::Ok({})", x),
        Err(_) => println!("Error"),
    }

    // Iterators (from prelude)
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("✓ Iterator with map: {:?}", doubled);

    // Commonly used traits
    println!("✓ Clone, Copy, Debug, Display, Drop, Eq, Ord, PartialEq, PartialOrd - all from prelude");

    // into(), to_string() - from prelude traits
    let num_string: String = 42.to_string();
    println!("✓ to_string(): {}", num_string);

    // println!, print!, eprintln! - macros from prelude
    println!("✓ println!, print!, eprintln! macros available");

    // panic!, assert!, assert_eq! - macros from prelude
    assert_eq!(2 + 2, 4);
    println!("✓ assert!, assert_eq! macros available");

    println!("\n=== All of the above work without explicit imports ===");
}
