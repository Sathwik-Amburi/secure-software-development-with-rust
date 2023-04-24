// use std::env;
// use std::process::Command;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let file_name = &args[1];

//     let output = Command::new("ls")
//         .arg(file_name)
//         .output()
//         .expect("Failed to execute command");

//     println!("{}", String::from_utf8_lossy(&output.stdout));
// }

// use std::env;
// use std::process::Command;
// use shell_escape::escape;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let file_name = &args[1];

//     let escaped_file_name = escape(file_name);

//     let output = Command::new("ls")
//         .arg(&escaped_file_name)
//         .output()
//         .expect("Failed to execute command");

//     println!("{}", String::from_utf8_lossy(&output.stdout));
// }

use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <directory>", args[0]);
        return;
    }

    let dir_name = &args[1];

    match fs::read_dir(dir_name) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        if let Some(file_name) = entry_path.file_name() {
                            println!("{}", file_name.to_string_lossy());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                eprintln!("Error: Directory not found");
            } else {
                eprintln!("Error: {}", e);
            }
        }
    }
}
