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


## Benefits of Rust

- Compile time memory safety - whole classes of memory bugs are prevented at compile time
- No undefined runtime behavior - what a Rust statement does is never left unspecified
- Modern language features - as expressive and ergonomic as higher-level languages, like enums and pattern matching, *generics*, and great compiler errors.

Important: *Generics* son una herramienta poderosa en la programación moderna que permiten escribir código más flexible, seguro y reutilizable al abstraer los tipos de datos con los que se trabaja. Tanto TypeScript como Rust (y muchos otros lenguajes como Java, C#, Go, Swift, etc.) hacen un uso extensivo de los genéricos, aunque con diferencias en su implementación y enfoque (como el borrado de tipos vs. la monomorfización).


## Macro Hygiene

A macro being ‘hygienic’ means that it doesn‘t accidentally capture identifiers from the scope they are used in.

Macros in Rust are **partially** hygienic, also called mixed hygiene. This means that they are hygienic when it comes to local variables, labels and $crate, but nothing else.
