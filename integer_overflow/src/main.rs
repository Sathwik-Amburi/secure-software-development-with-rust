use std::io;

fn main() {
    let mut balance = 100.0; // initial balance
    println!("Current balance: ${}", balance);

    // Prompt user for withdrawal amount
    println!("Enter withdrawal amount: ");
    let mut withdrawal_amount = String::new();
    io::stdin()
        .read_line(&mut withdrawal_amount)
        .expect("Failed to read line");

    // Parse withdrawal amount as a float
    let withdrawal_amount: f64 = withdrawal_amount.trim().parse().expect("Invalid input");

    // Check if withdrawal amount is greater than balance
    if withdrawal_amount > balance {
        panic!("Insufficient funds");
    }

    // Update account balance
    balance -= withdrawal_amount;
    println!("Withdrawal successful. New balance: ${}", balance);
}
