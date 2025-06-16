# Hello World Rust

Simple project for learning the Rust language.
It follows mainly ‘The Rust Programming Language’ book, and also some tutorial or course that, unfortunately, I don’t remember and didn’t documented.
Also, it follows the ‘Comprehensive Rust’ book from Google.

Note:
Rust fits in the same area as C++:
- High flexibility.
- High level of control.
- Can be scaled down to very constrained devices such as microcontrollers.
- Has no runtime or garbage collection.
- Focuses on reliability and safety without sacrificing performance.

## The Rust Ecosystem
As mentioned in the «Comprehensive Rust» book:
The Rust ecosystem consists of a number of tools, of which the main ones are:

- `rustc`: the Rust compiler which turns .rs files into binaries and other intermediate formats.
- `cargo`: the Rust dependency manager and build tool. Cargo knows how to download dependencies, usually hosted on https://crates.io, and it will pass them to rustc when building your project. Cargo also comes with a built-in test runner which is used to execute unit tests.
- `rustup`: the Rust toolchain installer and updater. This tool is used to install and update rustc and cargo when new versions of Rust are released. In addition, rustup can also download documentation for the standard library. You can have multiple versions of Rust installed at once and rustup will let you switch between them as needed.

According to «The Rust Programming Language»:
Cargo is Rust’s build system and package manager.
It uses Git as the default Version Control Syustem (VCS).
Creata new project with
```
cargo new project_name
```

Note: TOML means Tom’s Obvious, Minimal Language.
It is Cargo’s configuration format.

In Rust, packages of code are referred to as ‘crates’ (for the [dependencies] section).
