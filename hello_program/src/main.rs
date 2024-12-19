use std::io::{self, stdin};
const GLOBAL_VARIABLE:u8=100;
fn main() {
    println!("Hello, Coder!");

    let num:u8=5; //u8 is 8but unsigned integer and lenght between (0 to 255)
    //you can also use let num=5
    //it is inmutable

    //signed(+ or -) and unsigned integers(+)
    println!("The value stored in num is  {}",num);

    //for mutable 
    let mut num1=6;
    println!("The value stored in num1 is {}",num1);

    num1=55;
    println!("The value stored in num1 is {}",num1);

    //string 
    //&str
    // let s="Hi Coders!!";

    //string
    let s=String::from("Hi Coders!!");
    println!("This is string literal {}",s);


    //tupple
    let emp_info:(&str,u8)=("Rohit",25);
    
    let emp_name =emp_info.0;
    let emp_age=emp_info.1;

    // let(emp_name,emp_age)=("Rohit",25);
    println!("The name of employee is {} and age is {}",emp_name,emp_age);


    //functions
    print_value(5);

    let a:u8=10;
    let b:u8=20;
    let result:u8=add(a,b);
    println!("The sum of {} and {} is {}",a,b,result);


    let num:u8=11;
    let value=multiply_by_two(num);
    print!("num multipled by two is {}",value);


    let outside_variable=5;

    {
        let inside_variable=10;
        println!("Inside variable is {}",inside_variable);
        println!("Outside Variable is {}",outside_variable);

    }

    println!("Outside variable is {}",outside_variable);
    println!("{}",GLOBAL_VARIABLE);

    print_value1();


    //===>ownership

    println!("=====>Ownership<=====");
    let a1:String=String::from("Rohit");
    let a2:String=a1.clone();

    println!("s1 is {}",a1);
    println!("s2 is {}",a2);

    let x:u8=5;
    let y:u8=x;
    println!("This is x {}",x);
    println!("This is y {}",y);    

    let x:i32=5;
    process_integer(x);
    println!("This is x {}",x);


    //but for the string 
    let r:String=String::from("Rohit");
    precess_string(r.clone());
    println!("{}",r);


    //References
    let b1:String=String::from("hello");
    let len:usize=calculate_length(&b1);
    println!("The length of {} is {}",b1,len);


    //mutable references

    let mut b2:String=String::from("hello");
    append_world(&mut b2);
    println!("this is appended {}",b2);


    let b3=String::from("hello");
    let c1=&b3;
    let c2: &String=&b3;
    let c3: &String=&b3;

    println!("this is c1 {}, c2 {}, c3 {}",c1,c2,c3);


    //Dangling Reference

    // let reference_to_nothing=create_string_ref();

    //referencing & Derefrencing

    // let x=5;
    // let y=&x;
    // let z: &i32=&y;


    // let mut x=5;
    // let y=&mut x;
    // *y=50;

    // let mut x=5;
    // let mut y=&mut x;
    // *y=100;
    // let z=&mut y;
    // **z=120;
    // println!("x=()",x);


    //Auto Dereferencing 
    let r1=String::from("Heloo");
    let lenr1=calculate_lengthr1(&r1);
    println!("The length of {} is {}",r1,len);


    println!("====>Programming Concept<=====");
    //programming concepts

    //float type

    let float32_number:f32=3.14;
    let float64_number=6.28;

    println!("Float 32 number is {}",float32_number);
    println!("Float 64 number is {}",float64_number);

    //bool type
    let is_raining:bool=true;
    let is_sunny:bool=false;

    let need_umbrella=is_raining && !is_sunny;
    let need_glasses=is_raining || is_sunny;

    println!("need umberella is {},need glasses is {}",need_umbrella,need_glasses);


    //character type

    let letter_a:char='a';
    let emoji:char='👍';
    let kanji:char='日';

    println!("Letter a is {}",letter_a);
    println!("Emoji is {}",emoji);
    print!("Kanji is {}",kanji);


    //arrays

    let c1=[10,11,12];
    println!("a1 length is {},first element is {}",c1.len(),c1[0]);

    let mut c3=[10,11,12];
    c3[0]=100;
    println!("a3 length is {},first element is {}",c3.len(),c3[0]);

    for elem in c3.iter()
    {
        println!("{}",elem);
    }


    //vectors

    // let mut v:Vec<i32>=Vec::new();
    let mut v3= vec ![100,200,300];
    println!("vector = {:?}",v3);

    v3.push(15);
    println!("vector = {:?}",v3);

    v3.pop();
    println!("vector = {:?}",v3);

    //array read and write 
    let mut arr:[&str;2]=["Hello","World"];
    read_arr(&mut arr);
    write_arr(&mut arr);
    println!("{:?}",arr);

    //vector read and write 
    let mut vrr:Vec<&str>=vec!["Hello", "World"];
    read_vrr(&vrr);
    write_vrr(&mut vrr);
    println!("{:?}",vrr);

    //type inference

    let d1=5;
    let d2=6.5;
    let d3="Hello world";

    println!("type of x: {}",&d1);
    println!("type of x: {}",&d2);
    println!("type of x: {}",&d3);


    //shadowing
    let e1=5;
    println!("Original value of e1 is {}",e1);

    let e1="hello";
    println!("New value of e1 is {}",e1);

    let e1=e1.len();
    println!("length of e1 is {}",e1);


    //if else 

    let number =1;
    if number%2==0 && number %4==0
    {
        println!("The number is divisible by 2 and 4")
    }
    else if number%2==0
    {
        println!("The number is divisible by 2")
    }
    else if number%4==0
    {
        println!("The number is divisible by 4")
    }
    else
    {
        println!("The number is not divisible by 2 or 4")
        
    }

    //simple loop
//    loop {
//        println!("Loop")
//    }


   //while loop
   let mut count=0;
   while count<5
   {
    println!("Count:{}",count);
    count+=1;
   }

   //for loop
   let array=[1,2,3,4,5];

   for element in &array{
    println!("{}",element)
   }

   //match 
   let number=2;

   match number{
    1=>println!("one"),
    2=>println!("two"),
    3 | 4 | 5 =>println!("three,four,five"),
    _=>println!("other number")
   }

   //match with guard

   let m=6;
   match x{
    n if is_even(n)=>println!("Even"),
    n if !is_even(n)=>println!("Odd"),
    _=>println!("other")
   }


//    let result=add(3,4);
//    println!("result is {}",result);


//user input 

println!("Enter your Name: ");

let mut input=String::new();
io::stdin().read_line(&mut input);

println!("Hello {}!",input);



   







}













fn print_value(item:u8)
{
    println!("This is new fucntion ");
    println!("{}",item)
}

fn add(a:u8,b:u8)->u8
{
    return a+b;
}



fn multiply_by_two(item:u8)->u8
{
    return item*2;
}

fn print_value1()
{
    println!("{}",GLOBAL_VARIABLE)
}

fn process_integer(owned_integer:i32)
{
    println!("this is the owned integer {}",owned_integer);
}

fn precess_string(owned_string:String)
{
    println!("this is the owned string {}",owned_string);
}

fn calculate_length(s:&String)->usize
{
    return  s.len();
}

fn append_world(s:&mut String)
{
    s.push_str(" world");
}


// fn create_string_ref()-> &String{
//     let b4:String=String::from("hello");
//     return &b4;
// }


fn calculate_lengthr1(s:&String)-> usize
{
    return s.len();
}


fn read_arr(arr:&mut [&str;2])
{
    println!("{:?}",arr)
}

fn write_arr(arr:&mut [&str;2])
{
    arr[0]="New";
}

fn read_vrr(vrr:&Vec<&str>)
{
    println!("{:?}",vrr)
}

fn write_vrr(vrr:&mut Vec<&str>)
{
    vrr.push("New")
}

fn is_even(x:i32)->bool
{
    return x%2==0;
}


//modules import
// mod math
// {
//  pub fn add(a:i32,b:i32)->i32{
//      a+b
// }
// }

