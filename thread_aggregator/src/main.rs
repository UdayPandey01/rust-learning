use std::sync::mpsc;
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    
    let (tx, rx) = mpsc::channel();
    

    let mut thread_handles = Vec::new();

    let num_threads = 3;
    let chunk_size = data.len() / num_threads;

    for i in 0..num_threads {
        let thread_tx = tx.clone();
        
        let start = i * chunk_size;
        let end = start + chunk_size;
        let data_chunk = data[start..end].to_vec();

        let handle = thread::spawn(move || {
            let data_sum : i32 = data_chunk.iter().sum();
            tx.send(data_sum).unwrap();
            
        });
        thread_handles.Push(handle);
        
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let mut total_sum = 0;
    for _ in 0..num_threads {
        total_sum += rx.recv().unwrap();
    }
    

    println!("The total sum is: {}", total_sum);
}