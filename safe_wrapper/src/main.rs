extern "C" {
    fn cos(n: f64) -> f64;
}
fn rust_cos(n: f64) -> f64 {
    unsafe {
        cos(n)
    }
}


fn main() {
    let angle_rad = std::f64::consts::PI;
    
    let cosine_value = rust_cos(angle_rad);
    
    println!("The cosine of PI is: {}", cosine_value);
}