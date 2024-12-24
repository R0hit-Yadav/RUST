mod lifetimes;
mod traits;
//Generics
fn display<T:std::fmt::Debug>(value:T)
{
    println!("Value:{:?}",value);
}

//generics Strucrts
struct Point<T>
{
    x:T,
    y:T,
}


//generic enums
enum Result<T,E>
{
    Ok(T),
    Err(E),
}

//Generic Methods
struct Container;

impl Container{
    fn display<T:std::fmt::Debug>(value:T)
    {
        println!("Generic Value: {:?}",value);
    }
}

//Constranits on Generics
fn add<T: std::ops::Add<Output= T>>(a:T,b:T)->T
{
    a+b
}

//generic traits
trait Summable<T>
{
    fn sum(a:T,b:T)->T;

}
struct Math;

impl Summable<i32> for Math
{
    fn sum(a:i32,b:i32)->i32
    {
        a+b
    }
}


//combining Multiple Generics
struct Pair<T,U>
{
    first:T,
    second:U,
}


fn main() {
    display(42);
    display("hello");
    display(vec![1,2,3]);



    let int_point=Point{x:15,y:20};
    let float_point=Point{x:1.5,y:1.5};

    println!("Int point:({},{})",int_point.x,int_point.y);
    println!("Float Point:({},{})",float_point.x,float_point.y);




    let sucess:Result<i32,&str>=Result::Ok(200);
    let fail:Result<i32,&str>=Result::Err("Error");

    match sucess{
        Result::Ok(val)=>println!("Sucess:{}",val),
        Result::Err(err)=>println!("Error:{}",err),
    }
    match fail{
        Result::Ok(val)=>println!("Sucess:{}",val),
        Result::Err(err)=>println!("Error:{}",err),
    }


    Container::display(42);
    Container::display("hello");
    Container::display(vec![1,2,3]);


    let sum=add(5,10);
    let float_sum=add(1.5,1.5);
    println!("sum:{}",sum);
    println!("Float sum:{}",float_sum);


    let result=Math::sum(10,20);
    println!("Sum:{}",result);

    let pair=Pair{first:"Rust",second:2024};
    println!("Pair:({},{})",pair.first,pair.second);

    //traits
    traits::traits();
    
    lifetimes::main();



}
