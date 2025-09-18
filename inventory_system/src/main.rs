// enum Category {
//     Electronics,
//     Apparel,
//     Books
// }

// struct Product {
//     id : u32,
//     name : String,
//     price : f64,
//     category : Category
// }

// fn print_product_details(product: &Product) {
//     // TODO: Print the product's id, name, price, and category.
//     let category_string = match product.category {
//         Category::Electronics => "Electronics",
//         Category::Apparel => "Apparel",
//         Category::Books => "Books"
//     };
//     // Hint: You will not be able to print the category directly, as it's a custom enum.
//     // We'll learn the best way to do this (with `match`) in the next lesson.
//     // For now, let's just print the price and name.
//     println!("Product Name: {}", product.name);
//     println!("Price: ${}", product.price);
//     println!("Price: ${}", category_string);
// }

// fn main() {
//     let mut product = Product {
//         id : 1,
//         name : String::from("Uday"),
//         price : 400.0,
//         category : Category::Electronics,
//     };
//     print_product_details(&product)
// }

pub enum Command {
    Move { x: i32, y: i32 },
    Echo(String),
    Quit,
}

fn process_command (command: Command) -> String {
    match command {
        Command::Move {x,y} => {
            format!("Moving to x:{}, y:{}", x, y)
        }
        Command::Echo(s) => {
            s
        }
        Command::Quit => {
            String::from("Quitting")
        }
    }
}

fn main() {
    let move_cmd = Command::Move {x:10, y:20};
    let echo_cmd = Command::Echo(String::from("Echo"));
    
    println!("{}", process_command(move_cmd));
    println!("{}", process_command(echo_cmd));
}