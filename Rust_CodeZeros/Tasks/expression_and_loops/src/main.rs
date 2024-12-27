//Expressions and loops
// use std::collection::HashMap;
fn main() {

//=============>if 
// if statements are used to check conditions and execute code based on whether the condition is true or false.
// Rust's if is an expression, meaning it can return a value.
// You can chain multiple conditions using else if, and handle different conditions with else.
// The condition in an if statement must always evaluate to a boolean value (true or false).

    let number=10;
    if number>5
    {
        println!("Greater than 5");
    }
    

    let number=3;
    if number>5
    {
        println!("Greater than 5");
    }
    else
    {
        println!("Less than 5");
    }




    if number>10
    {
        println!("Greater than 10");

    }
    else if number>5 
    {
        println!("number is greater then 5 but less then 10");
    }
    else
    {
        println!("Less than 5 or equal");
    }



    let number=10;
    let result=if number>5
    {
        "Grater then 5"
    }
    else{
        "less then or equal to 5"
    };

    println!("{}",result);


    let number=20;
    if (number%2==0) && (number>10)
    {
        println!("number is even and greater then 10");
    }

    let numbers=[1,2,3,4,5];

    for &num in numbers.iter()
    {
        if num%2==0
        {
            println!("{} is even",num);
        }
        else{
            println!("{} is odd",num);
        }
    }

    //=============>loop
    let mut counter=0;
    loop
    {
        counter+=1;
        println!("Counter is {}",counter);

        if counter==5
        {
            break;
        }
    }

    let mut counter=0; //while loop
    while counter<5
    {
        while counter<5
        {
            println!("Counter is {}",counter);
            counter+=1;
        }
    }

    for i in 0..5 // for loop
    {
        println!("i:{}",i)

    }

    for i in 0..=5//range
    {
        println!("i:{}",i)
    }


    let arr=[10,20,30,40,50];//for with collection 
    for num in arr.iter()
    {
        println!("{}",num)
    }

    for i in 0..10
    {
        if i ==3
        {
            continue;
        }
        if i==8
        {
            break;
        }
        println!("i:{}",i);
    }


    let fruits = vec!["apple", "banana", "cherry"];

    for fruit in fruits.iter() {
        println!("{}", fruit);
    }


// loop: Creates an infinite loop that can be terminated with break.
// while: Continues looping as long as the condition is true.
// for: Iterates over a range or collection, preferred for handling known iterations.
// break: Exits the loop.
// continue: Skips to the next iteration.

    let number =vec![1,2,3,4,5];

    let squared_numbers:Vec<i32>=number.into_iter().map(|x| x*x).collect(); //into_iter() converts the vector into an iterator.
    //map(|x| x * x) applies a closure that squares each element. collect() collects the results into a new Vec<i32>.

    println!("{:?}",squared_numbers);

    // let mut map=HashMap::new()
    // map.insert("a",1);
    // map.insert("b",2);
    // map.insert("c",3);

    // let transformed_map:HashMap<_,_>=map.iter()
    // .map(|(key,&value)|(key,value * 2))
    // .collect();

    // println!("{:?}",transformed_map);


    //==================>Match
    let number=7;

    match number{
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _=>println!("other number")
    }
    

    let pair=(1,2);
    match pair{
        (0,y)=>println!("the first element is zero,and the second is {}",y),
        (x,0)=>println!("the first element is {},and the second is zero",x),
        _=>println!("this is some other pair"),
    }

    //matching on option 
    let some_number=Some(10);
    let no_number:Option<i32>=None;

    match some_number{
        Some(x)=>println!("We have a number: {}",x),
        None=>println!("No number"),

    }

    match no_number{
        Some(x)=>println!("we have a number: {}",x),
        None=>println!("No number"),
    }

    let number =3;
    match number
    {
        1 | 2 =>println!("One or Two"),
        3=>println!("Three"),
        _=>println!("other number"),
    }


    //For and range
    for i in (1..=5).rev()
    {
        println!("{}",i);
    }

    for i in (1..=10).step_by(2)
    {
        println!("{}",i);
    }

    // for loops are used to iterate over ranges, arrays, and other collections.
    // 1..5 creates a range that includes 1 but excludes 5.
    // 1..=5 creates a range that includes both 1 and 5.
    // .rev() reverses a range for reverse iteration.
    // .step_by(n) allows you to iterate with a specific step size.
    // You can iterate over arrays and vectors using the for loop with .iter() for arrays and directly for vectors.


}





