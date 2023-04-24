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

To ensure that your Rust code is secure and resilient to integer overflows, you should handle integer overflows explicitly and safely, rather than relying on undefined behavior or panicking. In our project, we have included examples of Rust code vulnerable to integer overflow attacks, based on TUM research, along with unit tests that check for unsafe Rust code and prompt the user on why their code is unsafe and how to fix it. We encourage you to use these examples and resources to learn more about secure software development with Rust.
