// fn main() {
//     let s = "Hello, Rust!";    // s is a string, which is immutable and has a fixed size stored in the stack

//     {
//         let x = "Hello, World!"; // x scope starts here
//     } // x scope ends here

//     println!("{}", s); // s is still valid here
//     // println!("{}", x); // This would cause an error because x is out of scope

//     let mut y = String::from("Hello");  // y is a mutable String, which is stored on the heap and can grow in size
//     y.push_str(", Rust!"); // y is mutable and can be modified
//     println!("{}", y); // y is still valid here

//     let mut s1 = String::from("Hello, Rustaceans!");
//     let mut s2 = s1.clone(); // s2 is a clone of s1, so both are valid and independent

//     s1.push_str(" Welcome to the Rust community!"); // s1 can still be modified
//     // s2 remains unchanged because it is a clone

//     s2.push_str(" Enjoy your journey!"); // s2 can also be modified independently
//     s2.push_str(" Let's learn together!"); 

//     println!("{}", s1);
//     println!("{}", s2);
// }



// fn main() {
//     // Create a String on the heap
//     let s = String::from("Hello, Rust!");
//     takes_ownership(s); // Ownership of s is moved to the function; s is no longer valid here

//     let s2 = gives_ownership(); // Function returns a String, and s2 takes ownership

//     let s3 = takes_and_gives_ownership(s2); // s2 is moved into the function, which returns it back as s3

//     let x = 5; // x is an i32, which implements the Copy trait
//     makes_copy(x); // x is copied into the function; x is still valid here

//     println!("x is still valid: {}", x); // x can be used because it was copied, not moved
//     // println!("s is {s}"); // This would cause an error: s was moved and is no longer valid

//     // println!("s2 is {s2}"); // This would cause an error: s2 was moved and is no longer valid
//     println!("s3 is {s3}"); // s3 is valid because ownership was returned
// }

// // Function that returns a String, transferring ownership to the caller
// fn gives_ownership() -> String {
//     let s = String::from("Hello, Rust!");
//     s // Ownership of s is returned
// }

// // Function that takes ownership of a String and drops it when done
// fn takes_ownership(s: String) {
//     println!("Taking ownership: {}", s);
// } // s is dropped here

// // Function that takes ownership of a String and returns it back
// fn takes_and_gives_ownership(s: String) -> String {
//     println!("Taking ownership and giving back: {}", s);
//     s // Ownership is returned to the caller
// }

// // Function that takes a copy of an i32 (Copy trait), so the original is still valid
// fn makes_copy(x: i32) {
//     println!("Making a copy: {}", x);
// }



fn main() {
    let name = String::from("Uday Pandey");

    let (name , len) = calculate_length(name);

    println!("The name length of: {} is {}", name, len); 
}

fn calculate_length(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}