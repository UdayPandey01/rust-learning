fn main() {
    let player_initial:char = 'U';
    let mut player_score:u32 = 0;
    let mut account_balance:f32 = 100.0;


    println!("Welcome, player {}!", player_initial);
    println!("Your starting score is: {}", player_score);
    println!("Your starting balance is: ${}", account_balance);


    println!("\n--- You won a match! ---");

    player_score += 500;
    account_balance += 25.50; 


    let player_score = "Hello";
    
    println!("\n--- Final Report ---");
    println!("Player status: {}", player_score);
    println!("Final balance: ${}", account_balance);
}