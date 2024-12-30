// // Synchronous Channels
// use std::sync::mpsc;
// use std::thread;
// fn main() {

//     let(tx,rx)=mpsc::channel();

//     let handle=thread::spawn(move||{
//         for i in 1..5
//         {
//             tx.send(i).unwrap();
//             println!("Sent {}",i);
//         }
//     });

//     for received in rx
//     {
//         println!("Received {}",received);
//     }

//     handle.join().unwrap();
// }



//asynchronous channles

use tokio::sync::mpsc;
use tokio::time::{sleep,Duration};

#[tokio::main]

async fn main()
{
    let (tx,mut rx)=mpsc::channel(10);

    tokio::spawn(async move{
        for i in 1..5
        {
            tx.send(i).await.unwrap();
            println!("Sent {}",i);
            sleep(Duration::from_millis(100)).await;
        }

    });

    while let Some(received)=rx.recv().await{
        println!("Received {}",received);
    }
}