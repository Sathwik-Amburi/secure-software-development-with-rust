use std::{fs::File, io::Read, thread, time::Duration};

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

fn main() {
    let filename = "test.txt";

    // Simulate high frequency file access
    loop {
        println!("Reading file...");
        let contents = read_file(filename);
        println!("File contents: {}", contents);
        thread::sleep(Duration::from_millis(500));
    }
}
