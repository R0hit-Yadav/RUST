// Synchronous Channels
use std::sync::mpsc;
use std::thread;
fn main() {

    let(tx,rx)=mpsc::channel();

    let handle=thread::spawn(move||{
        for i in 1..50
        {
            tx.send(i).unwrap();
            println!("Sent {}",i);
        }
    });

    for received in rx
    {
        println!("Received {}",received);
    }

    handle.join().unwrap();
}





//asynchronous channles

// use tokio::sync::mpsc;
// use tokio::time::{sleep,Duration};

// #[tokio::main]

// async fn main()
// {
//     let (tx,mut rx)=mpsc::channel(10);

//     tokio::spawn(async move{
//         for i in 1..15
//         {
//             tx.send(i).await.unwrap();
//             println!("Sent {}",i);
//             sleep(Duration::from_millis(100)).await;
//         }

//     });

//     while let Some(received)=rx.recv().await{
//         println!("Received {}",received);
//     }
// }





// //unbounded channels
// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main()
// {
//     let (tx,mut rx)=mpsc::unbounded_channel();

//     tx.send("hi my name is rohit and i am coder!!").unwrap();

//     if let Some(value)=rx.recv().await
//     {
//         println!("received {}",value);
//     }
// }





// //bounded channels
// use std::sync::mpsc::sync_channel;

// fn main()
// {
//     let(tx,rx)=sync_channel(2);

//     tx.send(1).unwrap();
//     tx.send(2).unwrap();
//     println!("Send 2 message");

//     for received in rx
//     {
//         println!("Received {}",received);
//     }
// }

