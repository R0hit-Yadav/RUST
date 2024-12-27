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


## Generics
Generics in Rust allow you to write code that works with many different types without sacrificing type safety. By using generics, you can define functions, structs, enums, or methods that are flexible and reusable.

# Key Benefits of Generics
Type Safety: Ensures the correct type is used.

Reusability: Write once, use for multiple types.

Flexibility: Allows custom behavior for different types.

Performance: Generics are monomorphized at compile time, 
meaning there’s no runtime performance cost.


