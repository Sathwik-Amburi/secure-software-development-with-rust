use std::io;
use std::process::Command;

fn main() {
    println!("Enter a command to manage your lights (on/off/dim):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let user_input = input.trim();

    let output = Command::new("sh")
        .arg("./custom_commands/myutility.sh")
        .arg(user_input)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", stdout);
}
