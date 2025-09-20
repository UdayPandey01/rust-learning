use tokio::time::{sleep, Duration, Instant};

// This function simulates a network download that takes time.
async fn download_file(filename: &str, duration_ms: u64) -> String {
    println!("Starting download for: {}", filename);
    sleep(Duration::from_millis(duration_ms)).await;
    let content = format!("Content of {}", filename);
    println!("Finished download for: {}", filename);
    content
}

// TODO: 1. Add the `tokio::main` macro to this function.
// TODO: 2. Make the `main` function `async`.
#[tokio::main]
async fn main() {               
    let start_time = Instant::now();

    let handle_a = tokio::spawn(download_file("file_a.txt", 2000));
    let handle_b = tokio::spawn(download_file("file_b.txt", 1000));

    let content_a = handle_a.await.unwrap();
    let content_b = handle_b.await.unwrap();

    let duration = start_time.elapsed();

    // If you did this correctly, the total time should be ~2 seconds, not 3.
    println!("\nTotal time taken: {:?}", duration);
    println!("File A content: {}", content_a);
    println!("File B content: {}", content_b);
}