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
    
}

//snake_case :hello_world -rust follow
