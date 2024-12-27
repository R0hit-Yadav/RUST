
//tokio - a powerfull and popular runtime
// tokio::main

// #[tokio::main]
// std::future::Future;
// async fn main()
// {
//     println!("hello coders!!")
// }


// //async-std
// async_std::task::block_on
// async fn main()
// {
//     println!("Running with async-std!")
// }

// // future 
// async fn hello_future() -> &'static str
// {
//     "Hello coders!!"
// }

// fn main()
// {
//     let fut =hello_future();
// }

// //awiat 
// async fn main()
// {
//     greet().await;
// }
// async fn greet()
// {
//     println!("hello");
//     async_task().await;//wait for task to finish
//     println!("Good bye");
// }

// async fn async_task()
// {
//     println!("Ruko jara sabar karo..")
// }


//asyncs with tokio

use tokio::time::{sleep,Duration};

async fn perform_task()
{
    println!("task started");
    sleep(Duration::from_secs(5)).await;//go to sleep
    println!("task completed");
}

#[tokio::main]
async fn main()
{
    println!("Startinf process");
    perform_task().await;
    println!("finish...");
}



