// Explicit lifetime annotations
fn longest <'a>(x:&'a str,y:&'a str)-> &'a str
{
    if x.len()>y.len()
    {
        x
    }
    else 
    {
        y
    }
}

//struct with 
pub fn main()
{

    let str1=String::from("Hello0");
    let str2=String::from("World!");
    let result=longest(&str1,&str2);
    println!("The longest string is {}",result);

}