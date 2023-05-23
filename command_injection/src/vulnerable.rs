use std::io;
use std::process::Command;

fn main() {
    println!("Enter a command to manage your lights (on/off/dim):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let user_input = input.trim();

    let command = format!("./custom_commands/myutility.sh {}", user_input);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", stdout);
}
