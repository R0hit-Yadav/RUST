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


    //====================>Arrays

    let numbers=[1,2,3,4,5];
    println!("{}",numbers[0]);
    println!("{:?}",numbers);

    let repeated=[0;5];
    println!("{:?}",repeated);

    let numbers=[10,20,30,40,50]; //itreating Over Array
    for num in numbers.iter()
    {
        println!("{}",num);
    }

    println!("{}",numbers.len());

    print_array(numbers);

    let slice=&numbers[1..4];//sclies
    println!("{:?}",slice);


    //====================>Tuples
    let tuple=(1,2,3,4,5);
    println!("{}",tuple.0);
    println!("{:?}",tuple);

    let tuple=(1,2,3,4,5);
    let(a,b,c,d,e)=tuple;
    println!("a is {},b is {},c is {},d is {},e is {}",a,b,c,d,e);

    let nested_tuple=((1,2,3),(4,5,6));
    println!("{:?}",nested_tuple);
    println!("{:?}",(nested_tuple.0).2);
    
    let single=(5,);

    println!("{:?}",single);

    let point:(i32,f64,&str)=( 1,2.3,"helloe");
    println!("{},{},{}",point.0,point.1,point.2);
    println!("{:?}",point);


    //==================ennum
    let dir=Direction::North;

    match dir {
        Direction::North=>println!("North"),
        Direction::South=>println!("South"),
        Direction::East=>println!("East"),
        Direction::West=>println!("West"),
    }

    let shape=Shape::Circle(5.0);

    match shape{
        Shape:Circle(raduis)=>println!("Circle with raduis {}",raduis),
        Shape:Rect(width,height)=>println!("Rectangle with width {} and height {}",width,height),
    }



    
   

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

fn print_array(arr:[i32;5])
{
    for num in arr.iter()
    {
        println!("{}",num);
    }
}


enum Direction{
    North,
    South,
    East,
    West
}

enum Shape{
    Circle(f64),
    Rect(f64,f64),

}