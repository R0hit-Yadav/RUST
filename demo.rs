mod simple_client;
mod simulated_distributed_client;
mod using_signatures;
use std::io;


pub fn main() {

    println!("Select Option to run ");
    println!("1. Simple Client ");
    println!("2. Simulated Distributed Client ");
    println!("3. Using Signatures ");

    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Enter vaild input");

    match input.trim()
    {
        "1"=>
        {
            
            simple_client::main();
        }
        "2"=>
        {

            simulated_distributed_client::main();
        }
        "3"=>
        {
            using_signatures::main();
        }
        _ =>
        {
            println!("Enter vaild input");
        }
    }

}



// ==========================> Un comment this⬇️⬇️ to run all in one 


// use rand::rngs::OsRng; // for random number
// use rand::RngCore; // to use fille_bytes
// use ed25519_dalek::{SigningKey, VerifyingKey, Signature}; // dalek library for signing and verifying
// use tokio::sync::mpsc;
// use tokio::time::{sleep, Duration}; // for sleep duration time
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message}; // for  connect socket
// use futures_util::StreamExt; // for  StreamExt
// use serde_json; // for json
// use std::env;
// use std::fs;
// use chrono::Local; //  local time
// use ed25519_dalek::{Signer, Verifier}; 

// #[tokio::main]
// pub async fn main() {
//     let input: Vec<String> = env::args().collect(); // get input from cmd

//     if input.len() < 2 {  // check if input is less then 2
//         println!("Invalid inputs!");
//         return;
//     }

//     match input[1].as_str() { // check which mode it it   
//         "--mode=cache" => {
//             let times = input
//                 .get(2)
//                 .and_then(|arg| arg.strip_prefix("--times="))
//                 .and_then(|val| val.parse::<u64>().ok())
//                 .unwrap_or(5);

//             distributed_client(times).await;
//         }
//         "--mode=read" => {
//             read_get_data();
//         }
//         _ => {
//             println!("Invalid inputs!");
//         }
//     }
// }

// async fn distributed_client(duration: u64) { // function to simulate distributed clients
//     let n_clients = 5;
//     let (tx, mut rx) = mpsc::channel::<(usize, f32, Signature)>(n_clients);

//     let mut signing_keys = vec![]; // vectors to store keys
//     let mut public_keys = vec![]; 

//     for i in 0..n_clients {  
//         let mut csprng = OsRng;  // generate signingKeys for all clients
//         let mut random_bytes = [0u8; 32]; // Ed25519 keys are 32 bytes
//         csprng.fill_bytes(&mut random_bytes); 
//         let signing_key = SigningKey::from_bytes(&random_bytes); 

//         public_keys.push(signing_key.verifying_key()); // store the public keys
//         signing_keys.push(signing_key); // store the signing keys
//     }

//     for i in 0..n_clients {
//         let tx = tx.clone(); // clone the sender for every client
//         let signing_key = signing_keys[i].clone(); // cllone the signing key

//         tokio::spawn(async move {
//             client_process(i, duration, tx, signing_key).await; // call the client process 
//         });
//     }

//     aggregator(&mut rx, n_clients, &public_keys).await; // call the aggregator
// }

// async fn client_process(
//     id: usize,
//     duration: u64,
//     tx: mpsc::Sender<(usize, f32, Signature)>,
//     signing_key: SigningKey,
// ) {
//     let req = "wss://stream.binance.com:9443/ws/btcusdt@trade"; // Request URL
//     let (mut socket, _) = connect_async(req).await.expect("Not connected"); // connect to the socket
//     println!("Client no {} is connected", id);
 
//     let mut price_vec = vec![]; // vec to store the prices
//     let start_time = Local::now(); // start time

//     while Local::now().signed_duration_since(start_time).num_seconds() < duration as i64 { 
//         if let Some(Ok(message)) = socket.next().await { // get the message from the socket
//             if let Message::Text(text) = message { // check if the message is text
//                 if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) { // read JSON
//                     if let Some(price) = parsed["p"].as_str() { // Get the price 'p'
//                         let price: f32 = price.parse().unwrap_or(0.0);
//                         price_vec.push(price); // Push the price to the vector
//                     }
//                 }
//             }
//         }
//         sleep(Duration::from_secs(1)).await; // Sleep for 1 second
//     }

//     let avg = price_vec.iter().sum::<f32>() / price_vec.len() as f32; // Calculate the average
//     sleep(Duration::from_secs(1)).await;
//     println!("Client no {} avg is: {}", id, avg);

//     let signature = signing_key.sign(&avg.to_be_bytes()); // Sign the average
//     tx.send((id, avg, signature)).await.expect("Failed to send");
// }

// async fn aggregator(                                  // Function to aggregate the data
//     rx: &mut mpsc::Receiver<(usize, f32, Signature)>,
//     n_clients: usize,
//     public_keys: &[VerifyingKey],
// ) {
//     let mut avg = vec![];

//     for _ in 0..n_clients {
//         if let Some((client_id, avg_price, signature)) = rx.recv().await {
//             if public_keys[client_id]
//                 .verify(&avg_price.to_be_bytes(), &signature)
//                 .is_ok()
//             {
//                 println!(
//                     "Aggregator verified and received avg {} from client no {}",
//                     avg_price, client_id
//                 );
//                 avg.push(avg_price);
//             } else {
//                 println!(
//                     "Aggregator could not verify signature for client no {}",
//                     client_id
//                 );
//             }
//         }
//     }

//     let overall_avg = avg.iter().sum::<f32>() / avg.len() as f32; // Calculate the overall average
//     println!("The overall avg BTC price in USD is: {}", overall_avg);

//     let data = format!("Client avg: {:?} Overall avg: {}", avg, overall_avg); // Store the data in a file
//     fs::write("aggregated_price_of_btc_in_usd.txt", data).expect("Cannot write to file");
// }

// fn read_get_data() { // Function to read the data from the file
//     let data = fs::read_to_string("aggregated_price_of_btc_in_usd.txt").expect("Cannot read file");
//     println!("Data of file is: {}", data);
// }