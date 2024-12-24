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

//struct with lifetimes
struct ImportantExcerpt<'a>
{
    part:&'a str,
}


//static lifetime
fn static_lifetime()-> &'static str
{
    "I am static!"
}

//lifetimes with traits
trait DisplayPart<'a>
{
    fn display(&self)->&'a str;
}
struct Text<'a>
{
    content:&'a str,
}

impl<'a> DisplayPart<'a> for Text<'a>
{
    fn display(&self)-> &'a str
    {
        self.content
    }
}

//lifetime in functions
fn combine_and_display<'a,T:std::fmt::Display>(x:&'a T,y:&'a T)-> &'a T
{
    println!("First:{},Second:{}",x,y);
    x
}

pub fn main()
{

    let str1=String::from("Hello0");
    let str2=String::from("World!");
    let result=longest(&str1,&str2);
    println!("The longest string is {}",result);


    let text=String::from("Hello World!");
    let excerpt=ImportantExcerpt{part:&text[0..5]};
    println!("excerpt:{}",excerpt.part);

    let s= static_lifetime();
    println!("{}",s);

    let text=Text{content:"Hello World!"};
    println!("{}",text.display());

    let a=10;
    let b=20;
    let result=combine_and_display(&a,&b);
    println!("Result:{}",result);

}