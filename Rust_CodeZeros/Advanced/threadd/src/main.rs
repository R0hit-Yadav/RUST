
//thread 
use std::thread;

//kill thread using atomic flag
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
// use std::time::Duration;


// how to kill task 
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

//thread_local! macro

thread_local!{
    static THREAD_ID:std::cell:cell::Cell<u32>=std::cell::Cell::new(0);
}


// //joining multiple task
// use tokio::join;

// async fn task1()
// {
//     println!("task 1 running...")
// }
// async fn task2()
// {
//     println!("task 2 running...")
// }

// #[tokio::main]
// async fn main()
// {
//     let(tx,mut rx)=watch::channel(false);

//     let task = tokio::spawn(async move {
//         loop
//         {
//             if *rx.borrow()
//             {
//                 println!("task Cancelled");
//                 break;
//             }
//             println!("task running...");
//             sleep(Duration::from_secs(1)).await;
//         }
//     });
//     sleep(Duration::from_secs(5)).await;
//     tx.send(true).unwrap();
//     task.await.unwrap();

//     //for join
//     let result=join!(task1(),task2());
//     println!("both tasks finished: {:?}",result);
// }







fn main() {
    let handle=thread::spawn(||{println!("from new thread!")});

    handle.join().unwrap();
    println!("Main thread finished");

    let handle1=thread::spawn(||{println!("Thread ID:{:?}",thread::current().id())});
    handle1.join().unwrap();


    let stop_signal=Arc::new(AtomicBool::new(false));
    let signal_clone=stop_signal.clone();

    let handle2=thread::spawn(move || {
        while !signal_clone.load(Ordering::Relaxed)
        {
            println!("Thread Running..");
            thread::sleep(Duration::from_millis(100));
        }
        println!("thread Stopping..");
    });

    thread::sleep(Duration::from_secs(2));
    stop_signal.store(true,Ordering::Relaxed);
    handle2.join().unwrap();


    //join multiple threads

    let mut handles=vec![];
    print

    for i in 0..5
    {
        handles.push(thread::spawn(move||{
            println!("Thread {} in running",i)
        }));
    }

    for handle3 in handles 
    {
        handle3.join().unwrap();
    }

    println!("all threads joined")

    fn main()
    {
        THREAD_ID.with(|id| id.set(1));
        let handle4=thread::spawn(||{
            THREAD_ID.with(|id|
            {
                id.set(2);
                println!("thread-local value:{}",id.get());
            });
        });

        handle4.join().unwrap();

        THREAD_ID.with(|id|
        {
            println!("main thread-local value:{}",id.get());

        });
    }

}
