use std::result;

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

    
}

fn print_value(item:u8)
{
    println!("This is new fucntion ");
    println!("{}",item)
}

fn add(a:u8,b:u8)->u8{
    return a+b;
}
//snake_case :hello_world -rust follow
