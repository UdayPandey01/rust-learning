// A struct to represent the author
struct Author {
    username: String,
    email: String,
}

// An enum for the post's status
enum PostStatus {
    Draft,
    Published,
    Archived,
}

// A struct for the blog post itself
struct Post {
    author: Author,
    title: String,
    content: String,
    status: PostStatus,
    likes: u32,
    cover_image_url: Option<String>,
}

fn main() {
    let author = Author {
        username: String::from("tech_guru"),
        email: String::from("guru@example.com"),
    };

    let post = Post {
        author: author,
        title: String::from("Mastering Rust Enums"),
        content: String::from("Enums are powerful..."),
        status: PostStatus::Published,
        likes: 1337,
        cover_image_url: Some(String::from("https://example.com/image.png")),
    };

    println!("Post '{}' has {} likes.", post.title, post.likes);

    if post.cover_image_url.is_some() {
        println!("Post has a cover image.");
    } else {
        println!("Post does not have a cover image.");
    }
}