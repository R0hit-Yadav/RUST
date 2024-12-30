//Box 

// //use box to store data on heap
// fn main()
// {
//     let value=Box::new(42);
//     println!("value stored in box:{}",value);
// }


// //box with recursive data structures
// enum List
// {
//     Cons(i32,Box<List>),
//     Nil,
// }

// use List::{Cons,Nil};

// fn main()
// {
//     let list=Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
//     println!("List Created With Box");

// }


// //box for trait objects
// trait Shape
// {
//     fn area(&self)->f64;
// }
// struct Circle
// {
//     radius:f64,
// }
// impl Shape for Circle
// {
//     fn area(&self)->f64
//     {
//         3.14*self.radius*self.radius
//     }

// }

// fn main()
// {
//     let shape:Box<dyn Shape>=Box::new(Circle{radius:10.0});
//     println!("Area:{}",shape.area());
// }


//box for reducing stack usage

fn main()
{
    let large_data=Box::new([1;10_000]);//stores a large array on the heap
    println!("First element of large data:{}",large_data[1001]);
}