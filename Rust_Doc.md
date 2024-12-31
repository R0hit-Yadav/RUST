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


# Locks 
Locks in Rust are used for safe concurrent programming to manage shared resources across threads or asynchronous tasks. Rust ensures data race safety at compile time by requiring explicit synchronization mechanisms like Mutex, RwLock, and Arc.

### Arc (Atomic Reference Counted)
Used to share ownership of immutable data between threads.

Provides thread-safe reference counting.

Immutable data is shared; mutability is not allowed directly.

Use Arc<Mutex<T>> to share mutable data safely.

### RefCell 
Provides interior mutability for single-threaded contexts.

Mutability is enforced at runtime.

Use RefCell<T> when you want to mutate data while borrowing it.

### Mutex
Ensures mutual exclusion for mutable access to shared data.

Only one thread can access the locked data at a time.

The lock is released when the guard (returned by lock()) goes out of scope.

### RwLock (Read-Write Lock)
Allows multiple readers or one writer at a time.

Ensures more efficient access when multiple threads need read-only access.


=>use cases for locks

Mutex: When only one thread needs access to modify data at a time.

RwLock: When multiple readers or a single writer are required for shared data.

Arc: For sharing immutable data across threads.

RefCell: For runtime-checked mutability in single-threaded contexts.



summary

Arc: Share immutable data between threads.

RefCell: Runtime-checked mutability in single-threaded contexts.

Mutex: Guarantees mutual exclusion for mutable access.

RwLock: Optimized for read-heavy workloads.

Async Locks: Non-blocking locks for async environments.



# Lifetime Spacifiers
Lifetime specifiers in Rust are a way to associate lifetimes (the scope during which a reference is valid) with references in functions, structs, or methods to ensure memory safety. Rust's ownership system ensures no dangling references, and lifetimes are the mechanism for explicitly declaring how references relate to each other.

### What Are Lifetime Specifiers?
Denoted by an apostrophe (e.g., 'a), a lifetime specifier associates a reference with a specific scope.

Helps Rust's borrow checker understand how long references are valid and prevent dangling pointers.


### Where Can They Be Used?
Function Signatures: To specify relationships between input and output lifetimes.

Struct Definitions: To associate the lifetime of struct fields with an external scope.

Traits and Implementations: To define lifetimes in trait bounds or implementations.


# Structs

## Static Methods and Object Methods in Rust 

### Satatic Methods
These are methods that do not take a reference to self as an argument.

They are associated with the type itself rather than a specific instance of the type.

Declared using impl StructName.

Invoked using the StructName::method_name() syntax.


### Object Methods 
These are methods that operate on a specific instance of the struct.

They take self (or &self/&mut self) as their first parameter.

Declared within the same impl block.

Invoked using the instance.method_name() syntax.


Static Methods:

No self parameter.

Used for tasks that don't depend on specific instances (e.g., constructors).


Object Methods:

Require self (or a reference to it).

Used for tasks specific to an instance.


# Box 
In Rust, a Box<T> is a smart pointer that provides heap allocation for a value of type T. Unlike regular variables, which are typically stored on the stack, Box stores its contents on the heap and keeps a pointer to the data.

### What is Box?
Box is a standard library type (std::boxed::Box) that allows you to store data on the heap.

It provides ownership of the heap-allocated value.

A Box is lightweight since it only contains a pointer to the heap memory.

### When is Box Used?
Heap Allocation: When you need to allocate data on the heap instead of the stack.

Recursive Data Structures: Enables defining recursive types that have indeterminate or unbounded sizes.

Trait Objects: Useful for enabling dynamic dispatch by storing trait objects.

Reducing Stack Size: Helps to offload large data from the stack to the heap.



Box is for heap allocation and single ownership.

Useful in recursive data structures and polymorphism.

It ensures memory safety through Rust's ownership model.



# Features in Rust 
In Rust, features are optional components or functionalities of a crate that can be enabled or disabled based on the needs of a project. Features allow crate authors and users to selectively compile parts of a crate, reducing binary size and improving performance by including only the required functionality.

### What are Features?
Features in Rust are compile-time configurations that allow conditional compilation of code.

Defined in a crate’s Cargo.toml file under the [features] section.

Can be used to enable or disable specific functionalities within a crate.

### Why Use Features?
Selective Functionality: Enables conditional compilation of code based on the user's needs.

Reduce Binary Size: Exclude unnecessary code to make binaries smaller and faster.

Flexibility: Add or remove functionalities without changing the core library.

Integration: Manage optional dependencies with features.



# Std Libs
Rust's standard library (std) provides essential tools for memory management, file operations, time manipulation, and advanced functional programming constructs. Below are the requested concepts with detailed explanations, examples, and outputs.

## std::mem
std::mem provides functions for memory manipulation, such as replacing, swapping, or forgetting values. It is particularly useful for working with values at a low level.

std::mem::replace Replaces the value at a mutable reference, returning the old value.

std::mem::swap Swaps two values.

std::mem::take Replaces a value with its default, returning the old value.

std::mem::forget Prevents a value from being dropped (use with caution to avoid leaks).


## std::fs
std::fs provides APIs for filesystem operations, such as reading, writing, and managing files and directories.


## std::time::Duration (Time Managment)

std::time::Instant ->Used for measuring time intervals.


## Differences: self, &self, &mut self
self: Consumes the instance, transferring ownership.

&self: Borrow the instance immutably.

&mut self: Borrow the instance mutably.

## Usecase of build.rs
A special script in Rust used for build-time customization, such as generating code or compiling native libraries.


## Functional Methods
Result

Option

and_then

ok_or_else

map

map_err

iter

iter_mut

into_iter


