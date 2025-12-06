fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause a compile-time error
    
    // To make a variable mutable, use `mut`
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is now: {}", y);
}