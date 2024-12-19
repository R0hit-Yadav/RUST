use std::io;
use rand::thread_rng;
use rand::prelude::*;
fn main() {
    loop{

   
    let guess_list=["apple","banana","orange"];
    let mut rng= thread_rng();

    let index =rng.gen_range(0..guess_list.len());
    let random_fruit=guess_list[index];

    println!("random fruit is {}",random_fruit);

    let mut input= String::new();
    match io::stdin().read_line(&mut input) 
    {

        Ok(_)=>{
            let fruit_selected=input.trim().to_lowercase();
            println!("you selected {}",fruit_selected);

            if !guess_list.contains(&fruit_selected.as_str())
            {

                println!("you selected wrong fruit");
                continue;
            }

            if guess_checker(&fruit_selected,random_fruit)
            {
                println!("congrats");
                break;
            }
            else
            {
                println!("try again");
            }
        

        }
        Err(error)=>
        {
            println!("error {}",error);        
        }
        
    }
}


}

fn guess_checker(guess_fruit:&str,random_selected:&str)->bool
{
    return guess_fruit==random_selected
}
