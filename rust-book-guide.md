# Guide to read Rust Book
- Book - https://doc.rust-lang.org/book/
- Read chapters and discuss with others on each checkpoint.
## 01. Install and Hello World.
- rustup - installer.
- rustc - compiler.
- cargo - package manager.
## 02. Hands on.
- _crate_ is a collection of Rust source code files.
- Cargo fetches the latest versions of everything from the _registry_, which is a copy of data from Crates.io.
## 03. Rust features.
Statements vs Expressions - evaluates to a value.
## 04. Ownership.
* Ownership rules:
 - Each value in Rust has a variable that’s called its owner.
 - There can only be one owner at a time.
 - When the owner goes out of scope, the value will be dropped.
   - _drop_: Resource Acquisition Is Initialization.
* Move
- s1 *moves* ownership to s2.
```
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```
* Clone
- deeply copy the heap data.
```
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```
* references:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
* Slices
- let you reference a contiguous sequence of elements in a collection.
- We can create slices using a range within brackets by specifying [starting_index..ending_index],
  - where starting_index is the first position in the slice and
  - ending_index is one more than the last position in the slice.

## 05. Structs and methods.

## 06. Enums and pattern match.
* Enum
- enum that can encode the concept of a value being present or absent. This enum is Option<T>
- you have to convert an Option<T> to a T before you can perform T operations with it
## [] 07. Package, crate, modules.
* module system:
- Packages: A Cargo feature that lets you build, test, and share crates.
  - one or more crates.
  - must contain zero or one library.
  - as many binary as you’d like.
- Crates: A tree of modules that produces a library or executable.
  - is a binary or library.
- Modules and use: Let you control the organization, scope, and privacy of paths.
  - organize code within a crate into groups.
- Paths: A way of naming an item, such as a struct, function, or module.
  - the use keyword that brings a path into scope;
  - the pub keyword to make items public.
 
## [] 08. Collections.
## [] 09. Error handling.
Recoverable (transient) and Unrecoverable (permanent)
Result<T,E> or panic!

Panic -> print failure, unwind and clean stack, quit.
RUST_BACKTRACE=1 env to show stacktrace.

enum Result<T,E> { Ok(T), Err(E) }
T,E generic types
helpermethods: unwrap -> return Ok(T) or panic. expect("my message for panic")
? operator: return Ok() to expression or Err() out of function.
- function should return Result or Option or implement ops:Try trait
* (good practice) Custom types for valition.


* Checkpoint
## [] 10. Generics, traits, lifetime.
* Checkpoint
## [] 11. Automated tests.
* Checkpoint
## [] 12. Project i/o.
* Checkpoint/MOB
## [] 13. Closures, iterators.
* Checkpoint
## [] 14. Cargo++.
* Checkpoint
## [] 15. Smart pointers.
* Checkpoint
## [] 16. Concurrent programming.
* Checkpoint
## [] 17. Idioms.
## [] 18. Patterns and pattern matching.
* Checkpoint
## [] 19. Unsafe, macros, lifetime++.
* Checkpoint
## [] 20. Project web server.
* Checkpoint
## [] 21. Appendix.
