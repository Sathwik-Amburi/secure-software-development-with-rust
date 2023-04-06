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
