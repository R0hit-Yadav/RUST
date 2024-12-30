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


# Thread 
A thread is the smallest sequence of programmed instructions that can be managed independently by a scheduler. In Rust, threads allow you to perform concurrent tasks by running them in parallel on different CPU cores. Rust provides a safe and efficient threading model to handle concurrency.

### What is Task ?
A task is an abstraction over a unit of work that may run asynchronously. Tasks can be managed by an async runtime, unlike threads that are managed by the operating system.
Tasks are generally cheaper than threads and are used in asynchronous programming.

### What is ThreadId ?
A ThreadId is a unique identifier for a thread in Rust. Each thread is assigned a unique ID that can be used for debugging or logging purposes.


### What is Multithreaded Programming ?
Multithreaded programming involves using multiple threads to execute tasks concurrently. Rust’s std::thread module provides tools for creating and managing threads safely.

Benefits
Increased performance by parallel execution.

Better utilization of CPU cores.

Challenges
Managing shared resources safely (handled by Rust using ownership and borrowing rules).

Avoiding deadlocks and race conditions.

### How to Kill a Thread ?
Rust does not provide direct APIs to kill threads, as threads are expected to terminate gracefully. Threads should be designed to stop based on signals or conditions.

### How to join multiple threads ?
The join method blocks the current thread until the target thread finishes execution. You can join multiple threads by storing their handles in a collection and iterating over them.

### how to kill a task ?
Tasks in Rust are typically managed by async runtimes and are not "killed" directly. Instead, you can use cancellation tokens or other signaling mechanisms.

### How to join Multiple Tasks ?
In asynchronous programming, you can use tokio::join! or futures::join! to join multiple tasks.

### thread_local! Macro?
The thread_local! macro creates thread-local storage, which means each thread gets its own copy of the data. It is useful when data is not meant to be shared across threads.

### lazy_static! Macro?
The lazy_static! macro is used to define global, lazily initialized static variables. It is often used for configurations, caches, or other static resources.


Feature ->                Description

Thread->	                A unit of execution managed by the OS.

Task->	                Lightweight unit of work managed by an async runtime.

ThreadId->	            Unique identifier for a thread.

Multithreading->	        Executing tasks concurrently using multiple threads.

Kill Thread/Task->	    Use signals or flags to stop execution gracefully.

Join Threads->	        Use join method to wait for threads to complete.

Join Tasks->	            Use tokio::join! or futures::join! to wait for multiple async tasks.

thread_local!->	        Provides thread-local storage for per-thread data.

lazy_static!->	        Creates global, lazily initialized static variables.


# Channels 
A channel in Rust is a mechanism for communication between threads or asynchronous tasks. It allows one thread or task to send data to another in a safe and synchronized manner. Rust channels are part of the message-passing model for concurrency.

### What is it Used For?
Communication between threads.

Sharing data between asynchronous tasks.

Decoupling producers and consumers.

Synchronizing execution of tasks or threads.


### Synchronous Channels (Sync)
The sending thread will block until the receiver consumes the message.

Useful for tightly synchronized communication.

Ensures that data is always handled before more is sent.

### Asynchronous Channels (Async)
The sender can continue without waiting for the receiver to process the message.

Useful for tasks where the sender doesn’t depend on immediate processing by the receiver.

Typically buffered, allowing a fixed number of messages to be queued.


### Unbounded Channels
Unlimited capacity; messages are stored until the receiver consumes them.

Can lead to high memory usage if the sender produces faster than the receiver consumes.


### Bounded Channels
Fixed capacity; if the buffer is full, the sender blocks (sync) or waits (async).

Prevents unlimited memory usage.


1.Use synchronous channels (std::sync::mpsc) for tightly synchronized communication between threads.

2.Use asynchronous channels (tokio::sync::mpsc) for loosely coupled async tasks.

3.Choose bounded channels to limit memory usage or unbounded channels for simplicity.

4.Advanced crates like Crossbeam offer more flexibility with channels.



# Closures 
A closure is a function-like construct that can capture variables from its surrounding environment. It is anonymous and can be stored in variables, passed as arguments, or returned from other functions. Closures in Rust are similar to lambdas in other programming languages.

|parameters| expression_or_block
|parameters|: The inputs to the closure.
expression_or_block: The body of the closure.


## Types of clsures

### Fn 
The closure only borrows variables from the environment.

It doesn’t modify or move captured values.

Closures implementing Fn can be called multiple times.

### FnMut
The closure can modify the environment by mutably borrowing variables.

It can be called multiple times.

### FnOnce
The closure takes ownership of the environment (moves captured variables).

It can only be called once because the environment is consumed.