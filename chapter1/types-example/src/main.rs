// Rust Type System Examples
// Rust is statically typed, but it can often infer types for you.

fn main() {
    println!("=== Rust Type System ===\n");

    // ===== INTEGERS =====
    println!("--- Integers ---");
    let x = 42;  // i32 is default integer type
    let y: u32 = 100;  // explicitly unsigned 32-bit
    let z: i64 = 9_223_372_036_854_775_807;  // max i64
    let a: u8 = 255;  // max u8
    println!("i32 (inferred): {}", x);
    println!("u32 (explicit): {}", y);
    println!("i64 (explicit): {}", z);
    println!("u8 (explicit): {}", a);

    // ===== FLOATS =====
    println!("\n--- Floating-point ---");
    let pi = 3.14;  // f64 is default float type
    let e: f32 = 2.718;  // explicitly 32-bit
    println!("f64 (inferred): {}", pi);
    println!("f32 (explicit): {}", e);

    // ===== BOOLEANS =====
    println!("\n--- Booleans ---");
    let is_active = true;
    let is_ready: bool = false;
    println!("is_active: {}", is_active);
    println!("is_ready: {}", is_ready);

    // ===== CHARACTERS =====
    println!("\n--- Characters (Unicode) ---");
    let c = 'A';
    let emoji: char = '🦀';
    let greek: char = 'α';
    println!("char: {}", c);
    println!("emoji: {}", emoji);
    println!("greek letter alpha: {}", greek);

    // ===== TUPLES =====
    println!("\n--- Tuples (fixed-size, mixed types) ---");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // destructure tuple
    println!("tuple: ({}, {}, {})", x, y, z);
    println!("access by index: {} {} {}", tup.0, tup.1, tup.2);

    // More tuple examples
    let person: (&str, i32) = ("Alice", 30);
    println!("person tuple: {:?}", person);

    // ===== ARRAYS =====
    println!("\n--- Arrays (fixed-size, same type) ---");
    let arr = [1, 2, 3, 4, 5];
    println!("array: {:?}", arr);
    println!("length: {}", arr.len());
    println!("first element: {}", arr[0]);

    // Explicit type and length
    let arr2: [i32; 3] = [10, 20, 30];
    println!("array with explicit type: {:?}", arr2);

    // Array initialized with same value
    let arr3 = [42; 5];  // five 42s
    println!("array [42; 5]: {:?}", arr3);

    // ===== TYPE INFERENCE =====
    println!("\n--- Type Inference ---");
    let num = 5;  // inferred as i32
    let sum = num + 1;  // still i32
    let product = num * 2;  // still i32
    println!("inferred i32 operations: {} + 1 = {}, {} * 2 = {}", num, sum, num, product);

    // Mixed types require explicit annotation
    let mixed: f64 = num as f64 + pi;  // cast num to f64
    println!("mixed (i32 as f64 + f64): {}", mixed);

    println!("\n=== Type System Summary ===");
    println!("✓ Integers: i32 (default), u32, i64, u8, etc.");
    println!("✓ Floats: f64 (default), f32");
    println!("✓ Booleans: bool");
    println!("✓ Characters: char (Unicode)");
    println!("✓ Tuples: Fixed-size, mixed types");
    println!("✓ Arrays: Fixed-size, same type");
}
