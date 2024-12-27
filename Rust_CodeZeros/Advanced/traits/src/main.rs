// traits

pub trait Greet{
    fn greet(&self)->String;
}
struct Person
{
    name:String,
}
impl Greet for Person{
    fn greet(&self)->String{
        format!("Hello {}",self.name)
    }
}

fn main()
{
    let person=Person{name:String::from("Traits")};
    println!("{}",person.greet());

}



//using Traits and Constraints
pub trait Addable
{
    fn add(&self,other:&Self)->Self;

}

impl Addable for i32
{
    fn add(&self,other:&Self)->Self
    {
        self+other

    }
}

fn sum<T:Addable>(a:T,b:T)->T
{
    a.add(&b)
}

fn main() {
    println!("{}", sum(5, 10)); // Output: 15
}

//=======>Key-builds
//sized 
fn example<T:Sized>(val:T)
{
}

//send
fn spawan_thread<T:Send>(value:T)
{
    std::thread
}

//sync
fn is_sync<T:Sync>()
{
    println!("T is sync");
}

//PartialEq
#[derive(PartialEq)]
struct Point
{
    x:i32,
    y:i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    println!("{}", p1 == p2); // Output: true
}

//Clone
#[derive(Clone)]
struct MyStruct
{
    value:String,
}

fn main()
{
    let original=MyStruct{Value:"Hello".to_String()};
    let copy=original.clone();
    println!("{}",copy.value); //hello
}

//debug
#[derive(Debug)]
struct MyStruct
{
    value:i32,
}
fn main() {
    let my_struct = MyStruct { value: 42 };
    println!("{:?}", my_struct); // Output: MyStruct { value: 42 }
}


//Display

use std::fmt;
struct MyStruct
{
    value:i32,
}

impl fmt::Display for MyStruct
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result
    {
        write!(f,"Value:{}",self.value)
    }
}

fn main()
{
    let my_struct=MyStruct{value:42};
    println!("{}",my_struct);//output: value:42
}

//serialize and Deserialize
//add to Cargo.toml
[dependencies]
serde={version="1.0",features=["derive"]}


use serde::{Serialize,Deserialize};
struct MyStruct
{
    value:i32,
}

fn main()
{
    let my_struct=MyStruct{value:42};
    let serialized=serde_json::to_string(&my_struct).unwrap();
    println!("{}",serialized); //output:{"Value":42}
}


//hash

use std::collections::HashMap;
#[derive(Hash,eq,PartialEq,Debug)]
struct MyStruct
{
    value:i32,
}

fn main()
{
    let mut map=HashMap::new();
    map.insert(MyStruct{value:1},"Answer");
    println!("{:?}",map)
}

//eq
#[derive(Eq,PartialEq)]
struct MyStruct
{
    value:i32,
}