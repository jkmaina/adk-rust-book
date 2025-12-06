fn main() {
    println!("Control flow example");

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Show two iterations
    for _ in 0..2 {
        println!("again!");
    }

    // While with mutable counter
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!");

    // For (Iterating)
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Loop that returns a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop returned: {}", result);

    // Match example
    let x = 5;
    match x {
        1 => println!("one"),
        2..=5 => println!("two through five"),
        _ => println!("something else"),
    }

    println!("done");
}
