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


## Loops

- `while`
- `for`
- `loop`: works ‘like’ a `while true`, until the condition of a `break` is met. Use it for things like servers that need to serve connections forever. Also, this is the only looping construct that can return a non-trivial value. This is because it’s guaranteed to only return at a `break` statement, unlike `while` and `for`, which can return when the condition fails, too.

### `break` and `continue`
(from Comprehensive Rust)

The same as other languages, `break` finishes the loop early and `continue` starts immediately the next iteration.

But, here is a special characteristic: *Labels*. They are used to break out of nested loops, for example:
```rust
fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    dbg!(elements_searched);
}
```
Here, the `break` command exites the `for` loop labeled `'outer`.

Labeled breaks also work on arbitrary blocks, e.g.:
```rust
'label: {
    break 'label;
    println!("This line gets skipped");
}
```


## Macros
(from Comprehensive Rust)

Macros are expanded into Rust code during compilation, and can take a variable number of arguments. They are distinguished by a ! at the end. The Rust standard library includes an assortment of useful macros.

- println!(format, ..) prints a line to standard output, applying formatting described in std::fmt.
- format!(format, ..) works just like println! but returns the result as a string.
- dbg!(expression) logs the value of the expression and returns it.
- todo!() marks a bit of code as not-yet-implemented. If executed, it will panic.
- assert! and related macros can be used to add assertions to your code. These are used heavily in writing tests.
- unreachable! is used to mark a branch of control flow that should never be hit.
- eprintln! allows you to print to stderr.


## Bonus

Here’s an interesting conversation part in a Reddit thread in r/haskell, called «Haskell vs Rust : elegant» (Haskell is another language I like very much):


n0body12345 PO • hace 1 a
Why doesn't Haskell perform so well on memory constrained contexts? I thought GHC was a pretty intelligent compiler (what I've seen thrown around).

Can something be done to make Haskell more performant in such contexts? Like how cpython etc do it for python maybe

Pentalis • hace 1 a
Haskell is garbage collected, Rust is not. The former will clog the scarce memory of embedded systems rather quickly and there's nothing the compiler can do about it; it's a problem with garbage collection in general. Also the device might need running continuously, and micro pauses for garbage collection may be completely off the table. Rust wont have any of those problems.
