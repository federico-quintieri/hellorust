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
    // 6
    arrays_function();
    // 7
    tuples_function();
    // 8
    slices_function();
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

// Compound Data Types
// arrays, tuples, slices, strigns (slice string)

// Arrays
fn arrays_function() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array numbers: {:?}", numbers);
}

// Tuples
fn tuples_function() {
    // Puoi anche non inserire i tipi di ogni elemento
    let human: (&str, i32, bool, [i32; 3]) = ("Alice", 30, false, [1, 2, 3]);
    println!("Human Tuple: {:?}", human);
}

// Slices
fn slices_function() {
    let number_slices: &[i32] = &[1, 2, 3, 4, 5, 6];
    println!("Number Slice: {:?}", number_slices);
}
