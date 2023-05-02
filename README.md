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

Time-of-check to time-of-use (TOCTOU) attacks are a class of race condition vulnerabilities that occur when a system's state changes between the time a resource is checked (e.g., for access permissions) and the time it is used. This can result in unauthorized access or privilege escalation. While Rust's focus on safety and concurrency can help mitigate many types of race conditions, TOCTOU attacks are still possible in Rust code, especially when interacting with external resources like file systems or databases like SQLite.

To understand TOCTOU attacks in Rust, let's consider an example involving SQLite:

1. A Rust program checks if a user has access to a specific SQLite database file.
2. Before the program opens the database file, another process (e.g., a malicious process) replaces the original file with a different SQLite database file with malicious content.
3. The Rust program, assuming it has checked the access permissions, now opens the new malicious SQLite database file instead of the original one.

Here, the TOCTOU attack has succeeded because the state of the system (the database file) changed between the time the Rust program checked access permissions and when it opened the file.

To prevent TOCTOU attacks in Rust, you can take the following steps:

1. `Use atomic operations`: Rust provides atomic types in the std::sync::atomic module, which can be used to create safe, lock-free concurrent data structures. These atomic operations can help ensure that the state of the system does not change unexpectedly between checking and using a resource.

2. `Lock resources`: If possible, lock the resource you're working with for the duration of the operation. For file system interactions, use file locking mechanisms (like flock() on Unix-like systems) to prevent other processes from modifying the file between the check and use. In the case of SQLite, SQLite's own file locking mechanism can help ensure that only one process can access the database file at a time.

3. `Use a single operation` for checking and using resources: If possible, combine the check and use steps into a single atomic operation. For example, instead of checking for a file's existence and then opening it, open the file directly and handle any errors (like ENOENT for a nonexistent file) that may arise.

4. `Reduce the time window between check and use`: Minimize the time between checking and using resources. While this doesn't completely eliminate the risk of TOCTOU attacks, it reduces the time window during which an attacker can intervene.

5. `Use proper access controls`: Ensure that your application enforces proper access controls, such as file system permissions and user authentication. This can limit the potential impact of a successful TOCTOU attack.

In summary, although Rust provides many safety and concurrency features, TOCTOU attacks are still possible, especially when interacting with external resources. To prevent these attacks, use atomic operations, lock resources, combine check and use steps into single operations, reduce the time window between checking and using resources, and enforce proper access controls.

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
