fn main() {
    // IMMUTABILITY: This value cannot be changed.
    let an_immutable_number = 42;
    println!("My lucky number is: {}", an_immutable_number);

    // MUTABILITY: This value can be changed because of the 'mut' keyword.
    let mut current_score = 0;
    println!("Starting score: {}", current_score);
    current_score = 100;
    println!("New score: {}", current_score);

    // --- DATA TYPES ---

    // TYPE INFERENCE: The compiler knows this is an integer (i32 by default).
    let year = 2025;
    println!("The year is {}", year);
    
    // TYPE ANNOTATION: We are explicitly telling the compiler the type.
    let version: f64 = 1.0;
    println!("Version: {}", version);

    let is_complete: bool = false;
    println!("Is the lesson complete? {}", is_complete);

    let initial: char = 'R';
    println!("The first letter of Rust is {}", initial);

    // Shadowing: You can declare a new variable with the same name.
    // This is different from a mutable variable! We are creating a brand new `year`.
    let year = "twenty twenty-five";
    println!("The year in words is {}", year);
}