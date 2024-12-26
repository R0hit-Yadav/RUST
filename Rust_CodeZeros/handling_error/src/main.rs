//recoverable error

fn divide(a:i32,b:i32)->Result<i32,String>
{
    if b==0
    {
        Err(String::from("Division by Zero is not allowed"))
    }
    else
    {
        Ok(a/b)
    }
}


//propagating error with ? oprator

use std::fs::File;
use std::io::{self,Read};

fn read_file_content(path:&str)->Result<String,io::Error>
{
    let mut file=File::open(path)?; //error if file not open
    let mut content=String::new();
    file.read_to_string(&mut content)?; //error if file not read
    Ok(content)
}


//custom error types
use std::fmt;

#[derive(Debug)]
enum MathError
{
    DIvisionByZero,
    NegativeNumber,
}

impl fmt::Display for MathError
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result
    {
        match self
        {
            MathError::DIvisionByZero=>write!(f,"Division by Zero is not allowed"),
            MathError::NegativeNumber=>write!(f,"Negative number is not allowed"),
        }
    }
}

fn divide1(a:f64,b:f64)->Result<f64,MathError>
{
    if b==0.0
    {
        Err(MathError::DIvisionByZero)
    }
    else
    {
        Ok(a/b)
    }
}


//using Option
fn find_item(items:&[i32],target:i32)->Option<usize>
{
    items.iter().position(|&x| x == target)
}











fn main() {

    match divide(10,2)
    {
        Ok(result)=>println!("Result:{}",result),
        Err(e)=>println!("Error:{}",e),
    }

    match divide(10,0)
    {
        Ok(result)=>println!("Result:{}",result),
        Err(e)=>println!("Error:{}",e),
    }
    

    //using unwrap and expct

    let value="123".parse::<i32>().unwrap();//works fine
    println!("Value:{}",value);

    // let value="abc".parse::<i32>().expect("Failed To parse");//panic with message


    match read_file_content("./src/hello.txt")
    {
        Ok(content)=>println!("Content:{}",content),
        Err(e)=>println!("Error:{}",e),
    }


    //panic!  uncoverable Errors
    // let numbers=vec![1,2,3];
    // println!("{}",numbers[10]);


    match divide1(10.0,0.0)
    {
        Ok(result)=>println!("result:{}",result),
        Err(e)=>println!("Error:{}",e),
    }


    

    let items=[1,2,3,4];
    match find_item(&items,3)
    {
        Some(index)=>println!("Index:{}",index),
        None=>println!("Item not found"),
    }



}
