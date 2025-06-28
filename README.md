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


### println! macro

println! macro receives:
- {} for the default output format
- {:?} for the debug output, it requires the type to implement the Debug trait
- {:b} for binary format
- {:x} for hexadecimal format
- {:o} for octal format
- {:e} for scientific notation
- {:p} for pointer address

Note: Arrays only implement debug output format

Adding `#`, eg `{a:#?}`, invokes a ‘pretty printing’ format, which can be easier to read.


## References
(from Comprehensive Rust)

- References can never be `null`, making them safe to use without `null` checks.
- References can’t outlive the data they point to.


### Shared References

A reference provides a way to access another value without taking ownership of the value, and is also called “borrowing”.
Shared references are read-only, and the referenced data cannot change.

A shared reference to a type `T` has type `&T`.
A reference value is made with the `&` operator.
The `*` operator “dereferences” a reference, yielding its value.

- References can never be null in Rust, so null checking is not necessary.
- Rust will auto-dereference in some cases, in particular when invoking methods (try `r.is_ascii()`). There is no need for an -> operator like in C++.
- A shared reference does not allow modifying the value it refers to, even if that value was mutable (check exclusive references).


### Exclusive references

Exclusive references, also known as mutable references, allow changing the value they refer to. They have type `&mut T`.
- ‘Exclusive’ means that only this reference can be used to access the value, no other references (shared or exclusive) can exist at a time.
The referenced value cannot be accessed while the exclusive reference exists.
- Note: `let mut x_coord: &i32` and `let x_coord: &mut i32` are very different.
The first one represents a shared reference which can be bound to different values, while the second represents an exclusive reference to a mutable value.


## Structs

Regular structs are the most commonly used.
Each field defined within them has a name and a type, and once defined can be accessed using example_struct.field syntax.
The fields of a struct share its mutability, so `foo.bar = 2;` would only be valid if foo was mutable.
Adding pub to a field makes it visible to code in other modules, as well as allowing it to be directly accessed and modified.

Tuple structs are similar to regular structs, but its fields have no names.
They are used like tuples, with deconstruction possible via `let TupleStruct(x, y) = foo;` syntax.
For accessing individual variables, the same syntax is used as with regular tuples, namely `foo.0`, `foo.1`, etc, starting at zero.

Unit structs are most commonly used as marker.
They have a size of zero bytes, but unlike empty enums they can be instantiated, making them isomorphic to the unit type `()`.
Unit structs are useful when you need to implement a trait on something, but don't need to store any data inside it.

### Named structs

Very similar to C and C++.
```rust
struct StructName {
    field1: type,
    field2: type,
}
```

Creation:
```rust
let struct1 = StructName {
    field1: Type::from("The type");
    field2: 10,
};
// We can copy almost all the fields of struct1 into struct2:
let field1 = Type::from("Struct 2 field 1");
let struct2 = StructName { field1: field1, ..struct1 };
```

### Tuple structs and Newtypes

```rust
struct Point(i32, i32);  // tuple struct, field names are not important
struct PoundsOfForce(f64);
struct Newtons(f64);  // single-field wrappers: Newtypes
```

Newtypes are a great way to encode additional information about the value in a primitive type, for example:
- The number is measured in some units: `Newtons` in the example above.
- The value passed some validation when it was created, so you no longer have to validate it again at every use: `PhoneNumber(String)` or `OddNumber(u32)`.

The example of Newtypes is a subtle reference to the [Mars Climate Orbiter](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter) failure.

### Enums and Type Aliases

A type alias creates a name for another type. The two types can be used interchangeably.
This is similar to a `typedef`.
```rust
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
```
A newtype is often a better alternative since it creates a distinct type.
Prefer `struct InventoryCount(usize)` to `type InventoryCount = usize`.


### `const` and `static`

Constants are evaluated at compile time and their values are inlined wherever they are used.
Only functions marked const can be called at compile time to generate const values. const functions can however be called at runtime.
```rust
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
```

Static variables will live during the whole execution of the program, and therefore will not move.
When a globally-scoped value does not have a reason to need object identity, `const` is generally preferred.
```rust
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

Because `static` variables are accessible from any thread, they must be `Sync`.
Interior mutability is possible through a `Mutex`, atomic or similar.

It is common to use `OnceLock` in a `static` as a way to support initialization on first use.
`OnceCell` is not `Sync` and thus cannot be used in this context.
Thread-local data can be created with the macro `std::thread_local`.


## Pattern Matching

### Irrefutable Patterns

Irrefutable, means that they will always match the value on the right hand side.

```rust
fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;

    // Ignore everything but the last element.
    let (.., c) = tuple;
}
```

With the `..` operator, it can be ignored the middle elements of a tuple too.
For example, `tuple: (char, i32, bool, u8) -> let (first, .., last) = tuple;`.
It also works with arrays, for example `array: [u8; 5] -> let [first, .., last] = array;`.


### Matching Values

Values are matched with *patterns*, that can be simple values or more complex conditions.
```rust
#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    }
}
```

#### @ syntax

Binds a part of a pattern to a variable. For example:

```rust
let opt = Some(123);
match opt {
    outer @ Some(inner) => {
        println!("outer: {outer:?}, inner: {inner}");
    }
    None => {}
}
```
In this example inner has the value 123 which it pulled from the Option via destructuring, outer captures the entire Some(inner) expression,
so it contains the full Option::Some(123).
This is rarely used but can be useful in more complex patterns.


## Bonus

Here’s an interesting conversation part in a Reddit thread in r/haskell, called «Haskell vs Rust : elegant» (Haskell is another language I like very much):


n0body12345 PO • hace 1 a
Why doesn't Haskell perform so well on memory constrained contexts? I thought GHC was a pretty intelligent compiler (what I've seen thrown around).

Can something be done to make Haskell more performant in such contexts? Like how cpython etc do it for python maybe

Pentalis • hace 1 a
Haskell is garbage collected, Rust is not. The former will clog the scarce memory of embedded systems rather quickly and there's nothing the compiler can do about it; it's a problem with garbage collection in general. Also the device might need running continuously, and micro pauses for garbage collection may be completely off the table. Rust wont have any of those problems.
