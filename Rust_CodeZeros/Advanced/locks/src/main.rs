// //Arc(Atomic Reference Counted)
// //sharing immutable data 
// use std::sync::Arc;
// use std::thread;
// fn main() {
//     let data=Arc::new(vec![1,2,3]);

//     let mut handles=vec![];
    
//     for i in 0..3
//     {
//         let data_clone=Arc::clone(&data);
//         let hadle=thread::spawn(move||{
//             println!("Thread {}:{:?}",i,data_clone);
//         });
//         handles.push(hadle);
//     }

//     for handle in handles
//     {
//         handle.join().unwrap();
//     }


// }


// // RefCell
// use std::cell::RefCell;
// fn main()
// {
//     let data=RefCell::new(vec![1,2,3]);
//     data.borrow_mut().push(4);  //mutates the vector
//     println!("{:?}",data.borrow()); // immutavke borrow
// }



// Mutex
use std::sync::{Arc,Mutex};
use std::thread;
fn main()
{
    let counter=Arc::new(Mutex::new(0));
    let mut handles=vec![];

    for _ in 0..10
    {
        let counter=Arc::clone(&counter);
        let handle=thread::spawn(move||{
            let mut num=counter.lock().unwrap();
            *num+=1;

        });
        handles.push(handle);
    }
    for handle in handles
    {
        handle.join().unwrap();
    }
    println!("Result:{}",*counter.lock().unwrap());
}
