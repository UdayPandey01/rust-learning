#[derive(Debug)] 
struct Book {
    title: String,
    author: String,
    is_available: bool,
}

fn check_out_book(library: &mut Vec<Book>, title: &String) -> bool {
    for book in library.iter_mut() {
        if book.title == *title {
            if book.is_available {
                book.is_available = false;
                return true;
            } else {
                return false;
            }
        }
    }
    false
}

fn print_library_status(library: &Vec<Book>) {
    println!("{:<30} | {:<20} | {}", "Title", "Author", "Available");
    println!("{}", "-".repeat(65));
    for book in library {
        println!(
            "{:<30} | {:<20} | {}",
            book.title, book.author, book.is_available
        );
    }
}


fn main() {
    let mut library = vec![
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik"),
            is_available: true,
        },
        Book {
            title: String::from("The Hobbit"),
            author: String::from("J.R.R. Tolkien"),
            is_available: true,
        },
    ];

    let book_to_check_out = String::from("The Hobbit");

    println!("--- Initial Library Status ---");
    print_library_status(&library);

    println!("\nChecking out '{}'...", book_to_check_out);
    if check_out_book(&mut library, &book_to_check_out) {
        println!("Book checked out successfully.");
    } else {
        println!("Failed to check out book.");
    }

    println!("\n--- Final Library Status ---");
    print_library_status(&library);
}