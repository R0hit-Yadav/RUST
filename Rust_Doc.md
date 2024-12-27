# Rust 

# Trait 
A trait in Rust is a collection of methods that types can implement. It defines shared behavior and serves as a way to enforce consistency across types. Traits are similar to interfaces in other languages but are more powerful and flexible.

1.Sized - Indicates that a type has a known size at compile time.

2.Send- Indicates that a type can be transferred between threads.

3.Sync - Indicates that a type can be safely shared between threads

4.PartialEq -Provides functionality for comparing types for equality.

5.Clone -Allows types to explicitly create a deep copy.

6.Debug -Provides functionality to format the output for debugging.

7.Display -Allows custom formatting of output.

8.Serialize and Deserialize -Used with the serde crate for serializing and deserializing data structures.

9.Hash- Allows types to be used as keys in hash maps by implementing a hash function.

10.Eq -Indicates that a type has reflexive equality. It extends PartialEq.


Traits define shared behavior in Rust and can be implemented for any type.

Traits are essential for generic programming and enforcing constraints.

Rust's standard library provides a rich set of traits like Sized, Send, Clone, etc.

Deriving traits (e.g., #[derive(Debug, Clone)]) simplifies implementation.


# Generics
Generics in Rust allow you to write code that works with many different types without sacrificing type safety. By using generics, you can define functions, structs, enums, or methods that are flexible and reusable.

## Key Benefits of Generics
Type Safety: Ensures the correct type is used.

Reusability: Write once, use for multiple types.

Flexibility: Allows custom behavior for different types.

Performance: Generics are monomorphized at compile time, 
meaning there’s no runtime performance cost.


# Async 
A runtime in Rust provides the necessary environment to execute asynchronous tasks.

## Popular Async Runtimes in Rust
### Tokio
A powerful and popular runtime.

Provides support for async I/O, networking, and timers 

Used for building scalable network services.

### async-std
Similar to Rust's std library but designed for asynchronous programming.

Provides utilities for async file I/O, networking, and more.

### Smol
A lightweight async runtime.

Focused on minimalism and ease of use.

### Actix

A framework/runtime often used for web services.

### What is await?
The await keyword is used to pause execution of an async function until the associated Future is ready to produce a value.

async fn:Defines an asynchronous function that returns a Future.

Async programming in Rust revolves around Futures and is powered by runtimes like Tokio and async-std.

The async keyword turns functions into Futures, while await pauses execution until the Future resolves.

Runtimes manage task scheduling and execution efficiently.

Rust’s async system ensures high performance without compromising safety.