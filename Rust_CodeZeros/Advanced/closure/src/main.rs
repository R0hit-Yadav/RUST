//closure



// fn main() 
// {
//     let square =|x| x*x;
//     println!("Square of 5 is :{}",square(5));


//     let factor =3;
//     let multiply=|x| x*factor;
//     println!("3 * 4 = {}",multiply(4));
  

// }



// //types pf clsures

// // fn : immutable Borrow
// fn apply<F>(closure:F)
// where F:Fn()
// {
//     closure();
// }

// fn main()
// {

//     let message="Hello Coders!!";
//     let greet =|| println!("{}",message); //immutable borrow
//     apply(greet);// can be called multiple times
//     apply(greet);// NO issue with multiple calls

// }




// //FnMut:mutale Borrow
// fn apply<F>(mut closure:F)
// where F:FnMut()
// {
//     closure();
//     closure();
//     closure(); // can called multiple times

// }

// fn main()
// {
//     let mut count=0;
//     let mut increment=||{
//         count+=1;
//         println!("Count:{}",count);
//     };//mutable borrow

//     apply(increment);
// }





//FnOnce:Ownership
fn consume<F>(closure:F)
where F:FnOnce()
{
    closure();
}

fn main()
{
    let name=String::from("Rust");
    let consume_name=|| println!("Consumed: {}",name);//moves 'name'
    consume(consume_name);
}



//differences

//Fn it can not be changes and  call multiple times
// fn main() {
//     let x = 10;
//     let closure = || println!("Value: {}", x);
//     closure();
//     closure();
// }

//FnMut it can be chnages and call multiple times
// fn main() {
//     let mut x = 10;
//     let mut closure = || {
//         x += 1;
//         println!("Value: {}", x);
//     };
//     closure();
//     closure();
// }


//FnOnce it can be chnages and call once
// fn main()
// {
//     let data=String::from("Rust");
//     let closure=||println!("Data: {}",data);
//     closure();
// }