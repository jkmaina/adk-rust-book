fn main() {
    print_labeled_measurement(5, 'h');
    let num = 10;
    let incremented = plus_one(num);
    println!("{} plus one is {}", num, incremented);
}
 
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
 
// Functions with return values (arrow -> syntax)
fn plus_one(x: i32) -> i32 {
    x + 1 // No semicolon means this is an expression that returns a value
}
