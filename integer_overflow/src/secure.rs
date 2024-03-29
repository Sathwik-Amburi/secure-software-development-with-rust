use std::io;

fn main() {
    let mut balance: u32 = 1000;
    loop {
        println!("Your balance is: {}", balance);
        println!("Please input withdrawal amount: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let amount: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if amount > 0 {
            match balance.checked_sub(amount) {
                Some(new_balance) => balance = new_balance,
                None => {
                    println!("Error: withdrawal amount causes integer overflow");
                    continue;
                }
            }
        } else {
            println!("Error: invalid amount");
            continue;
        }
    }
}
