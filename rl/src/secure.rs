use std::{
    fs::File,
    io::{BufReader, Read},
    thread,
    time::Duration,
};

fn read_file(filename: &str) -> String {
    let file = File::open(filename).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
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
