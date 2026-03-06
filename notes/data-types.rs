// Data Types in Rust

fn main(){
    // Scalar types
    // Integer
    let signed: i32 = -42; // signed integer
    let unsigned: u32 = 42; // unsigned integer
    let isize: isize = 42; // Size depends on the architecture (32-bit or 64-bit)
    let usize: usize = 42; // Size depends on the architecture (32-bit or 64-bit)

    let decimal: i32 = 98_222; // Decimal
    let hex: i32 = 0xFF; // Hexadecimal
    let octal: i32 = 0o77; // Octal
    let binary: i32 = 0b1111_0000; // Binary
    let byte: u8 = b'A'; // Byte (u8 only)
    
    // Operations on integers
    let sum: i32 = 5 + 10; // Addition
    let dif: i32 = 10 -5; // Subtraction
    let product: i32 = 4 * 5; // Multiplication
    let quotient: i32 = 10 / 2; // Division
    let remainder: i32 = 10 % 3; // Remainder

    // Floating-point numbers
    let float: f32 = 3.14; // 32-bit floating point
    let double: f64 = 2.71828; // 64-bit floating point

    // Boolean
    let is_true: bool = true; // Boolean value
    let is_false: bool = false; // Boolean value

    // Character
    let letter: char = 'A'; // Character type
    let emoji: char = '😊'; // Unicode character
    
    // Compound types
    // Tuple
    let tuple: (i32, f64, char) = (2026, 3.14, 'A'); // Tuple with different types
    let (x, y, z) = tuple; // Destructuring a tuple

    let year = tuple.0; // Accessing tuple elements by index
    let pi = tuple.1; // Accessing tuple elements by index

    // Array
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array of integers with fixed size
    let first = array[0]; // Accessing array elements by index
    let second = array[1]; // Accessing array elements by index

    let mut mutable_array: [i32; 3] = [10, 20, 30]; // Mutable array
    mutable_array[0] = 15; // Modifying an element in the mutable array

    // Slice
    let slice: &[i32] = &array[1..4]; // Slice of the array (elements 2, 3, and 4)
    let first_in_slice = slice[0]; // Accessing slice elements by index

    // Strings
    let string : String = String::from("Hello World!"); // Owned string
    let str_sliced: &str = "Hello Rust!"; // String slice (Borrowed string)
    let mut mutable_str: String = String::from("Hello"); // Mutable String
    mutable_str.push_str(" Dev!"); // Appending to a string

    // String concatenation
    let concatenated = mutable_str + " Are you good?"; // Concatenating strings

    // if you do not set the type of the variable, the type will be inferred automatically
}