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


### Let Control Flow

```rust
if let  // executes different code depending on whether a value matches a pattern
while let  // repeatedly tests a value against a pattern
let else  // for the common case of matching a pattern and returning from the function
```

- Unlike `match`, `if let` doesn’t have to cover all branches, making it more concise in some cases.
- `if let` common usage: handling `Some` values when working with `Option`.
- With an `else` clause, `if let` can be used as an expression.

- `while let` loop will keep going as long as the value matches the pattern.
- `while let` loop can be rewrited as an infinite `while` loop with an `if` statement that breaks when there is no value to unwrap for `name.pop()`.
The `while let` provides syntactic sugar for the above scenario.
- `while let` cannot be used as an expression, because it may have no value if the condition is false.

- The `let else` early return-based control flow is common in Rust error handling code, where you try to get a value out of a `Result`,
  returning an error if the `Result` was `Err`.


## Methods and Traits

### Methods

Functions are associated to new types with the `impl` block.
```rust
struct Car {
    name: String,
}
impl Car {
    // No receiver, static method
    fn new(name: &str) -> Self {
        Self { name: String::from(name) }
    }

    // Exclusive borrowed read-write access to self
    fn change_name(&mut self, new_name: &str) {
        self.name = new_name;
    }

    // Shared and read-only borrowed access to self
    fn print_name(&self) {
        println!("Recorded name:", self.name);
    }

    // Exclusive ownership of self (covered later)
    fn finish(self) {
        println!("Car life of car {} is finished", self.name);
    }
}
```

Notes on `self`:
- `Self` is a type alias for the `impl` block it is in.
- `self` is an abbreviation of `self: Self`.
- Beyond variants on `self`, there are also special wrapper types allowed to be receiver types, such as `Box<Self>`.

The `self` arguments specify the “receiver” - the object the method acts on. There are several common receivers for a method:
- `&self`: borrows the object from the caller using a shared and immutable reference. The object can be used again afterwards.
- `&mut self`: borrows the object from the caller using a unique and mutable reference. The object can be used again afterwards.
- `self`: takes ownership of the object and moves it away from the caller. The method becomes the owner of the object. The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted. Complete ownership does not automatically mean mutability.
- `mut self`: same as above, but the method can mutate the object.
- No receiver: this becomes a static method on the struct. Typically used to create constructors which are called new by convention.


### Implementing traits

Similar to interfaces:
```rust
trait Pet {
    /// Return a sentence from this pet.
    fn talk(&self) -> String;

    /// Print a string to the terminal greeting this pet.
    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}
```

Traits may be implemented like this:

```rust
struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
    // greet has a default implementation
    // in this case, greet relies on talk. When fido calls greet, its name will be Fido!
}

fn main() {
    let fido = Dog { name: String::from("Fido"), age: 5 };
    dbg!(fido.talk());
    fido.greet();
}
```

Multiple `impl` blocks are allowed for a given type.
This includes both inherent `impl` blocks and trait `impl` blocks.
Likewise multiple traits can be implemented for a given type (and often types implement many traits!).
`impl` blocks can even be spread across multiple modules/files.

### Supertraits

With the `: Trait` “keyword”, a trait can require that types implementing it also implement other traits, called *supertraits*.
For example, in this case, Pet requires that types implementing it also implement Animal:
```rust
trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    fn name(&self) -> String;
}
```

### Associated types

Associated types are placeholder types which are supplied by the trait implementation.

```rust
#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

fn main() {
    println!("{:?}", Meters(10).multiply(&Meters(20)));
}
```

- Associated types are sometimes also called “output types”. The key observation is that the implementer, not the caller, chooses this type.
- Many standard library traits have associated types, including arithmetic operators and `Iterator`.

### Deriving

```rust
#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default(); // Default trait adds `default` constructor.
    let mut p2 = p1.clone(); // Clone trait adds `clone` method.
    p2.name = String::from("EldurScrollz");
    // Debug trait adds support for printing with `{:?}`.
    println!("{p1:?} vs. {p2:?}");
}
```


## Generics

Generics let you abstract algorithms or data structures (such as sorting or a binary tree) over the types used or stored.
For example, let’s see a pair of functions that function the same way, but for different types:

```rust
fn pick_i32(cond: bool, left: i32, right: i32) -> i32 {
    if cond { left } else { right }
}

fn pick_char(cond: bool, left: char, right: char) -> char {
    if cond { left } else { right }
}
```

The same logic can be abstracted into a more general function that works for both types:

```rust
fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond { left } else { right }
}
```

### Trait bounds

It is often required that the types considered in the generics implement a specific trait, to call the trait’s methods.

```rust
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

struct NotCloneable;  // This struct doesn’t implement any trait

fn main() {
    let foo = String::from("foo");  // String implements the trait Clone
    let pair = duplicate(foo);
    println!("{pair:?}");
}
```

#### `where` clause

```rust
fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}
```
- It declutters the function signature if you have many parameters.
- It has additional features making it more powerful: the type on the left of “:” can be arbitrary, like `Option<T>`.


### Generic data types

Generics can be used over the concret field type.

```rust
struct VerbosityFilter<L> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}
```

The type “L” must implement the trait “Logger”.

Generally, in Rust we put the trait bounds on the `impl` blocks, but can also be put on the type itself.

### Generic traits

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    dbg!(from_int);
    dbg!(from_bool);
}
```

### `impl Trait`

Similar to trait bounds, an `impl Trait` syntax can be used in function arguments and return values:
It allows to work with types which you cannot name.

```rust
// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}
fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}
```

### `dyn Trait`

Rust also supports using `dyn Traits` for type-erased, dynamic dispatch via trait objects.
A dyn Trait is considered to be “type-erased”, because we no longer have compile-time knowledge of what the concrete type is.


## Bonus

Here’s an interesting conversation part in a Reddit thread in r/haskell, called «Haskell vs Rust : elegant» (Haskell is another language I like very much):


n0body12345 PO • hace 1 a
Why doesn't Haskell perform so well on memory constrained contexts? I thought GHC was a pretty intelligent compiler (what I've seen thrown around).

Can something be done to make Haskell more performant in such contexts? Like how cpython etc do it for python maybe

Pentalis • hace 1 a
Haskell is garbage collected, Rust is not. The former will clog the scarce memory of embedded systems rather quickly and there's nothing the compiler can do about it; it's a problem with garbage collection in general. Also the device might need running continuously, and micro pauses for garbage collection may be completely off the table. Rust wont have any of those problems.
