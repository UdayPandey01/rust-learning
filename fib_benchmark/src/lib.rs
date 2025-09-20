// A slow, recursive implementation
pub fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

// A fast, iterative implementation
pub fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c;
    if n == 0 {
        return a;
    }
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    b
}