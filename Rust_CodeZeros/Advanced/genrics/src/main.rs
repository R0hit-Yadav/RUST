//generics in functions
fn largest<T:PartialOrd>(list:&[T])->&T{
    let mut largest=&list[0];
    for item in list.iter()
    {
        if item >largest{
            largest=item;
        }
    }
    largest

}

//in structs
struct Point<T>
{
    x:T,
    y:T,
}

struct Point2<T,U>
{
    x:T,
    y:U,
}

//in enums
#[derive(Debug)]
enum Option<T>
{
    Some(T),
    None,
}

enum Result<T,E>
{
    Ok(T),
    Err(E),
}

//in methods

struct Point3<T>
{
    x:T,
    y:T,
}

impl<T> Point3<T>
{
    fn x(&self)->&T
    {
        &self.x
    }
}



fn main() {

    let num=vec![1,2,13,4,15,6];
    println!("Largest number is {}",largest(&num)); // largest :15

    let char=vec!['a','b','c','d','e','f'];
    println!("Largest char is {}",largest(&char));//largest:f

    let int_point=Point{x:15,y:20};
    let float_point=Point{x:1.5,y:1.5};

    println!("({},{})",int_point.x,int_point.y);
    println!("({},{})",float_point.x,float_point.y);

    let mixed_point=Point2{x:1.5,y:2};
    println!("({},{})",mixed_point.x,mixed_point.y);

    let some_num=Option::Some(42);
    let some_s=Option::Some("r");

    println!("{:?},{:?}",some_num,some_s);


    let sucess:Result<i32,&str>=Result::Ok(200);
    let fail:Result<i32,&str>=Result::Err("Error");

    match sucess
    {
        Result::Ok(val)=>println!("Success:{}",val),
        Result::Err(e)=>println!("Fail:{}",e),
    }

    match fail
    {
        Result::Ok(val)=>println!("Success:{}",val),
        Result::Err(e)=>println!("Fail:{}",e),
    }

    let point3=Point3{x:1,y:2};
    println!("{}",point3.x());
}
