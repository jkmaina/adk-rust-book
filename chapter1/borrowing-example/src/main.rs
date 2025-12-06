fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass a reference
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
