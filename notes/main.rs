fn main(){
    // Immutable variable
    let x = 5;
    println!("The value of x is {} and it is immutable", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is {} and it is mutable", y);

    // Constants are always immutable and must be annotated with a type
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {} and it is a constant", MAX_POINTS);

    // Shadowing allows you to reuse the same variable name
    let z = 15;
    println!("The value of z is {} and it is immutable", z);
    let z = z + 5; // Shadowing the previous z
    println!("The value of z is now {} and it is still immutable", z);

    // Inner scope allows you to create a new variable with the same name
    {
        let x = 20; // This x shadows the outer x
        println!("The value of x in the inner scope is {} and it is immutable", x);
    }

    let z = "Hello, world!"; // Shadowing z with a different type
    println!("The value of z is now '{}' and it is a string", z);

    let spaces: &str = "     "; // A string of spaces
    let spaces: usize = spaces.len(); // Shadowing to change type from &str to usize
    println!("The value of spaces is {} and it is the length of the string", spaces);
}