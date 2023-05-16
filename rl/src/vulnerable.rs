use libc::{c_char, c_int, open, read, O_RDONLY};
use std::{ffi::CString, thread, time::Duration};

fn read_file(filename: &str) -> Result<String, String> {
    let c_filename = CString::new(filename).expect("CString::new failed");

    let fd: c_int = unsafe { open(c_filename.as_ptr() as *const c_char, O_RDONLY) };

    if fd < 0 {
        return Err("Unable to open file".to_string());
    }

    let mut buffer = vec![0u8; 1024];
    let bytes_read = unsafe { read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };

    if bytes_read < 0 {
        return Err("Unable to read file".to_string());
    }

    // Remove extra null bytes from the buffer
    buffer.resize(bytes_read as usize, 0);

    // Comment the line below to introduce resource leakage
    // unsafe { close(fd) };

    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

fn main() {
    let filename = "test.txt";

    loop {
        println!("Reading file...");
        match read_file(filename) {
            Ok(contents) => println!("File contents: {}", contents),
            Err(err) => println!("Error: {}", err),
        }
        thread::sleep(Duration::from_millis(50));
    }
}
