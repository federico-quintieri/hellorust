// Funzione che printa un hello world
fn main() {
    // 1
    println!("Hello, Rust from CARGO!");
    // 2
    integer_function();
    // 3
    float_function();
    // 4
    boolean_function();
    // 5
    char_function();
}

// Primitive data types
// int, float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned integer (only +) types of different sizes.
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integers
fn integer_function() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
}

// Floats [Floating Point Types]
// f32, f64
fn float_function() {
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);
}

// Boolean Values: true, false
fn boolean_function() {
    let t: bool = true;
    println!("Value of t: {}", t);
}

// Character Type - char
fn char_function() {
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}
