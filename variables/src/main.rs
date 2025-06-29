const AGE1 : u32 = 27; // Immutable constant

fn main() {
    const AGE2: u32 = 30;
    let mut age = 21;
    println!("My age is {}", age);

    age = 22; // Shadowing
    println!("My age is now {}", age);

    println!("Constant age is {AGE1}"); // Using constant   
    println!("Constant age is {AGE2}"); // Using constant

    const HOUR_IN_SECONDS: u32 = 60 * 60 + AGE1;
    println!("One hour in seconds is {HOUR_IN_SECONDS} seconds");

    const HOUR_IN_SECONDS2: u32 = 60 * 60 + age; // This will cause an error because `age` is not a constant
}
