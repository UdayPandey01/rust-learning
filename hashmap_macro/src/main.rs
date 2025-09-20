use std::collections::HashMap;

// TODO: Define the `hashmap!` macro using `macro_rules!`.
// The macro should accept a list of key-value pairs separated by commas.
// Example invocation: hashmap!{ "key1" => 1, "key2" => 2 }
// Hint: The pattern for a key-value pair is `$key:expr => $value:expr`.
// You'll need to use the repetition operator `$( ... ),*`.

macro_rules! hashmap {
    () => {
        HashMap::new()
    };

    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut temp_hash = HashMap::new();

            $(
                temp_hash.insert($key,$value);
            )*

            temp_hash
        }
    }
}


fn main() {
    // When your macro is complete, this code should compile and run.
    let scores = hashmap!{
        "Blue".to_string() => 10,
        "Yellow".to_string() => 50
    };

    println!("Created HashMap: {:?}", scores);
    assert_eq!(scores["Blue"], 10);
}