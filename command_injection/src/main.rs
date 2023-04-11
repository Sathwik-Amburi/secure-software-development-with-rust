use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let output = Command::new("ls")
        .arg(file_name)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
