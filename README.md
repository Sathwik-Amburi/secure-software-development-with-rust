# Secure Software Development with Rust

This project aims to provide resources and tools to help developers write secure Rust code. It focuses on common vulnerabilities that can be exploited by attackers, such as SQL injection, command injection, integer overflow, TOCTOU vulnerabilities, resource leakage, typoSquatting, authentication, and business logic.

## Getting Started

To get started with this project, you can clone the repository and run the following command:

```bash
git clone git@gitlab.com:Sathwik-Amburi/secure-software-development-with-rust.git
```

## Structure

The project is organized into folders for each topic, where you can find example code that contains unsafe Rust code and unit tests that check for unsafe Rust code. The tests will prompt the user on why their code is unsafe and how to fix it. The following is a brief overview of the folders:

- **sql-injection**: Contains examples of Rust code vulnerable to SQL injection attacks.
- **command-injection**: Contains examples of Rust code vulnerable to command injection attacks.
- **integer-overflow**: Contains examples of Rust code vulnerable to integer overflow attacks, based on TUM research.
- **toctou**: Contains examples of Rust code vulnerable to TOCTOU (time-of-check to time-of-use) attacks.
- **resource-leakage**: Contains examples of Rust code vulnerable to resource leakage.
- **typosquatting**: Contains examples of Rust code vulnerable to typoSquatting attacks.
- **authentication**: Contains examples of Rust code vulnerable to authentication attacks.
- **business-logic**: Contains examples of Rust code vulnerable to business logic attacks.

## Contributing

If you would like to contribute to this project, please follow the following steps:

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Commit your changes.
5. Push your changes to your fork.
6. Create a merge request.

## Integer Overflows in Rust

In Rust, integer overflows can occur when the result of an arithmetic operation on an integer type exceeds the maximum value that can be represented by the type. By default, Rust will perform a panic when an integer overflow occurs. However, Rust provides several ways to handle integer overflows explicitly and safely, such as using the `wrapping_*`, `checked_*`, and `overflowing_*` methods.

The `wrapping_*` methods perform the operation and wrap around the integer if an overflow occurs. The `checked_*` methods return an Option that contains the result of the operation if it does not overflow, or None if an overflow occurs. `The overflowing_*` methods return a tuple containing the result of the operation and a boolean flag indicating whether an overflow occurred.

To ensure that your Rust code is secure and resilient to integer overflows, you should handle integer overflows explicitly and safely, rather than relying on undefined behavior or panicking. In our project, we have included examples of Rust code vulnerable to integer overflow attacks, based on TUM research, along with unit tests that check for unsafe Rust code and prompt the user on why their code is unsafe and how to fix it. We encourage you to use these examples and resources to learn more about secure software development with Rust.\

## TOCTOU Vulnerabilities in Rust

### introduction

TOCTOU (Time-of-Check to Time-of-Use) is a class of race conditions that occur when the state of a system changes between the moment a value is checked and the moment it is used. In this guide, we will discuss TOCTOU vulnerabilities in Rust, their potential consequences, and how to prevent them.

### What is a TOCTOU vulnerability?

A TOCTOU vulnerability arises when a program checks a condition or a resource's state, and then, based on the result of the check, takes an action that depends on the resource's state. If the resource's state changes between the check and the subsequent action, the program's behavior can become unpredictable or insecure.

In Rust, TOCTOU vulnerabilities can occur when working with file system operations, shared memory, or any other resource that can be accessed or modified concurrently by multiple threads or processes.

### Consequences of TOCTOU Vulnerabilities

TOCTOU vulnerabilities can have several negative effects, including:

1. `Data corruption`: Concurrent access to shared resources can lead to inconsistent or corrupted data, especially if multiple threads or processes are modifying the resource simultaneously.
2. `Security vulnerabilities`: TOCTOU vulnerabilities can be exploited by an attacker to gain unauthorized access to resources, modify data, or cause a denial of service. For example, an attacker might be able to replace a file or a symbolic link between the time it is checked and the time it is used, leading to unexpected behavior or security issues.

### Preventing TOCTOU Vulnerabilities in Rust

Preventing TOCTOU vulnerabilities in Rust involves careful handling of shared resources, synchronization, and error handling. Here are some general guidelines to prevent TOCTOU vulnerabilities in Rust:

1. `Avoid using unsafe functions`: Some functions in Rust, such as std::fs::metadata, can introduce TOCTOU vulnerabilities if not used carefully. Instead, prefer using safe alternatives that handle the check and the action atomically, such as std::fs::OpenOptions.

For example, instead of checking if a file exists and then opening it, use OpenOptions to open the file directly:

```rust
use std::fs::OpenOptions;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).open("example.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);
    Ok(())
}

```

2. `Use synchronization mechanisms`: When working with shared resources, use synchronization mechanisms like Mutex, RwLock, or channels to ensure that only one thread or process can access the resource at a time, preventing race conditions.

For example, when working with a shared file, use a Mutex to ensure that only one thread can access the file at a time:

```rust
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file = Arc::new(Mutex::new(File::create("shared.txt").unwrap()));

    let mut handles = vec![];

    for _ in 0..10 {
        let file_clone = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file = file_clone.lock().unwrap();
            // Perform file operations...
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

```

3. `Handle errors gracefully`: Proper error handling is essential for preventing TOCTOU vulnerabilities. Ensure that your code correctly handles errors and recovers from unexpected changes in the state of shared resources. Rust's error handling mechanisms, such as Result and the ? operator, can help you write more robust code that handles shared resources correctly.

```rust
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};

fn read_or_create_file(file_path: &str) -> Result<String, Error> {
    let mut file = match OpenOptions::new().read(true).write(true).open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create(file_path)?;
            new_file.write_all(b"Initial content")?;
            new_file
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
```

4. `Use atomic operations`: When working with shared memory or data structures, use atomic operations provided by Rust's standard library, such as AtomicBool, AtomicUsize, or AtomicPtr. These operations are designed to be executed in a single step, preventing race conditions that could arise from concurrent access.

For example, use AtomicUsize to safely increment a shared counter:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.load(Ordering::SeqCst));
}
```

### Conclusion

TOCTOU vulnerabilities can lead to data corruption, security vulnerabilities, or unexpected behavior in a Rust program. To prevent these issues, it is crucial to handle shared resources carefully, using synchronization mechanisms, error handling, and atomic operations. By following these guidelines and Rust's best practices, you can write more robust and secure Rust applications that effectively manage shared resources and prevent TOCTOU vulnerabilities.

## Resource Leakage in Rust

### introduction

Resource leakage is a common issue in software development where system resources, such as memory, file handles, or network connections, are not properly released after being used. This can lead to various problems, such as performance degradation, system instability, or even security vulnerabilities.

In this guide, we will discuss resource leakage in Rust, its potential consequences, and how to prevent it.

### What is resource leakage?

Resource leakage occurs when a program does not correctly release a system resource after it has been used, leading to the resource being unavailable to other parts of the system or other programs. Common examples of resources that can be leaked include memory, file handles, and network connections.

In Rust, resource leakage can be caused by improper handling of memory allocation and deallocation, not closing file handles, or not managing network connections correctly.

### Consequences of Resource Leakage

Resource leakage can have several negative effects on a system, including:

1. `Performance degradation`: Leaking resources, such as memory or file handles, can cause a system to become slow or unresponsive over time, as the number of leaked resources accumulates.
2. `System instability`: In extreme cases, resource leakage can lead to system crashes or hangs, as the system becomes unable to allocate new resources due to resource exhaustion.
3. `Security vulnerabilities`: Resource leakage can lead to security vulnerabilities, such as information disclosure or denial of service, if sensitive data is not properly cleaned up or if an attacker is able to exhaust system resources.

### Preventing Resource Leakage in Rust

Rust's memory safety guarantees and its ownership system help prevent many common resource leakage issues. However, it is still possible to leak resources in Rust, especially when dealing with unsafe code or external resources like file handles or network connections. Here are some general guidelines to prevent resource leakage in Rust:

1. `Use RAII (Resource Acquisition Is Initialization) pattern`: In Rust, the RAII pattern is commonly used to manage resources by tying their lifetime to the lifetime of an object. When the object goes out of scope, the resource is automatically released. This can help ensure that resources are properly released, even in the presence of errors or early returns.

For example, the std::fs::File struct in the Rust standard library automatically closes the file handle when it goes out of scope:

```rust
use std::fs::File;

fn main() {
    let file = File::open("example.txt").unwrap();
    // Perform file operations...

    // File is automatically closed when `file` goes out of scope.
}

```

2. `Properly handle unsafe code`: When using unsafe code, it is essential to ensure that resources are properly managed. Be cautious when working with raw pointers, memory allocation, or external libraries, and always follow best practices and documentation for the code you are working with.

For example, when working with raw memory allocation, ensure that memory is properly deallocated after use:

```rust
use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};

let layout = Layout::from_size_align(size_of::<u32>(), align_of::<u32>()).unwrap();
let ptr = unsafe { alloc(layout) as *mut u32 };

if !ptr.is_null() {
    unsafe {
        *ptr = 42;

        // Perform operations with the allocated memory...

        // Properly deallocate the memory
        dealloc(ptr as *mut u8, layout);
    }
}
```

3. `Clean up sensitive data`: When working with sensitive data, such as encryption keys or passwords, it is important to ensure that the data is properly zeroized (overwritten with zeros or another constant value) before deallocating or releasing the associated memory. This helps prevent information disclosure by ensuring that sensitive data is not left behind in memory after it is no longer needed.

In Rust, the zeroize crate provides a secure and efficient way to zeroize sensitive data, taking care to avoid compiler optimizations that could potentially remove the zeroization:

```rust
use zeroize::Zeroize;

struct SensitiveData {
    data: [u8; 64],
}

impl SensitiveData {
    fn new() -> Self {
        SensitiveData {
            data: *b"Sensitive information: secret_key=ABC123!\0",
        }
    }
}

impl Zeroize for SensitiveData {
    fn zeroize(&mut self) {
        self.data.zeroize();
    }
}

fn main() {
    let mut sensitive_data = SensitiveData::new();

    // Perform operations with the sensitive data...

    // Properly zeroize the sensitive data before it goes out of scope
    sensitive_data.zeroize();
}

```

4. `Handle errors gracefully`: Proper error handling is essential for preventing resource leakage. Ensure that your code correctly handles errors and releases resources even in the case of failure. Rust's error handling mechanisms, such as Result and ? operator, can help you write more robust code that releases resources correctly.

```rust

use std::fs::File;
use std::io::{Read, Error};

fn read_file_contents(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    // File handle is properly closed even if an error occurs during reading
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

```

### Conclusion

Resource leakage can lead to performance issues, system instability, and security vulnerabilities. In Rust, the language's memory safety guarantees and ownership system help prevent many common resource leakage problems. However, it is still essential to follow best practices for managing resources, handling unsafe code, cleaning up sensitive data, and handling errors. By doing so, you can write more robust and secure Rust applications that effectively manage system resources.
