use std::io;
use std::process::Command;
mod shared;

fn main() {
    let script_path = "./custom_commands/myutility.sh";
    shared::make_script_executable(&script_path);

    println!("Enter a command to manage your lights (on/off/dim):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let user_input = input.trim();

    let command = format!("{} {}", script_path, user_input);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", stdout);
}
