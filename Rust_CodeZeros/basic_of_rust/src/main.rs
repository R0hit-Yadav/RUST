struct Person{
    name:String,
    age:u8,
    is_student:bool,
}
struct Point(i32,i32,i32);

fn main() 
{
    //=================>variables
    //in rust defult immutable variables

    let x=10;
    println!("{}",x);
    let mut y=20;
    println!("{}",y);
    y=30;
    println!("{}",y);

    //const never change
    const val:f32=3.14;
    println!("{}",val);

    //shandowing 
    let x=5;
    let x=x+1;
    let x=x*2;
    println!("{}",x);


    //=================>structs
    //add mut to change value
    let person=Person{
        name:String::from("Rohit"),
        age:25,
        is_student:true,
    };
    println!("{},{},{}",person.name,person.age,person.is_student);

    let point=Point(1,2,3);
    println!("{},{},{}",point.0,point.1,point.2);


    //=================>Functions
    greet();
    greet1("Rohit");
    let result=greet2(1,2);
    println!("sum is {}",result);
    //rust do not support overloading
    //closures
    let square=|x:i32| x*x;
    println!("Square of 3 is {}",square(3));

    //with multiple parameters
    let(sum,mul)=greet3(2,5);
    println!("sum is {},mul is {}",sum,mul);

    println!("5! is {}",greet4(5));


    //====================>Array
    


    
   

}


fn greet()
{
    println!("Hello, Coder!");
}

fn greet1(name:&str)
{
    println!("Name is {}",name);
}

fn greet2(a:i32,b:i32)->i32{
    a+b
}

fn greet3(a:i32,b:i32)->(i32,i32)
{
    (a+b,a*b)
}

fn greet4(n:u32)->u32{
    if n==0
    {
        1
    }
    else
    {
        n*greet4(n-1)
    }
}
