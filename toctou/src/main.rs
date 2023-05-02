use std::fs::{self, File};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";

    // Check if the file exists
    if fs::metadata(file_path).is_ok() {
        // Time gap between the check and the use
        std::thread::sleep(std::time::Duration::from_millis(10000));

        // Read the contents of the file
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        println!("File contents: {}", contents);
    } else {
        println!("File does not exist.");
    }

    Ok(())
}

// use std::fs::OpenOptions;
// use std::io::Read;

// fn main() -> std::io::Result<()> {
//     let file_path = "example.txt";

//     match OpenOptions::new().read(true).open(file_path) {
//         Ok(mut file) => {
//             let mut contents = String::new();
//             file.read_to_string(&mut contents)?;
//             println!("File contents: {}", contents);
//         }
//         Err(_) => {
//             println!("File does not exist.");
//         }
//     }

//     Ok(())
// }
