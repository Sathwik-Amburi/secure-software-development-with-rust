use std::io;
use std::process::Command;

fn main() {
    println!("Enter a file name:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let filename = input.trim();

    // let command = format!("rm -rf test-folder/{} file.txt", filename);
    let command = format!("ping -c4 {}", filename);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    println!("Output: {:?}", output);
}

// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     println!("Enter a file name:");
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read input");
//     let filename = input.trim();

//     match File::open(&filename) {
//         Ok(mut file) => {
//             let mut contents = String::new();
//             file.read_to_string(&mut contents)
//                 .expect("Failed to read file");
//             println!("File contents:\n{}", contents);
//         }
//         Err(e) => {
//             println!("Error opening file: {}", e);
//         }
//     }
// }
