fn main() {
    // Swap Two numbers

    let mut a = 5;
    let mut b = 10;
    println!("Before swap: a = {}, b = {}", a, b);

    let temp = a; 
    a = b;
    b = temp;
    println!("After swap: a = {}, b = {}", a, b);


    // Returns a tuple (sum, product) of two numbers
    fn sum_product(x : i32,y : i32) -> (i32, i32){
        let sum = x + y;
        let product = x * y;
        (sum, product)
    }

    println!("Sum and Product of 5 and 10: {:?}", sum_product(a, b));


    // function to check whether a number is prime or not

    fn check_prime(num : i32) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..=((num as f64).sqrt() as i32) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    println!("Is 5 prime? {}", check_prime(5));
}
