// //using lifetime specifiers in functions
// fn longest<'a>(x:&'a str,y:&'a str)->&'a str
// {
//     if x.len() > y.len()
//     {
//         x
//     }
//     else
//     {
//         y
//     }
// }

// fn main()
// {
//     let s1=String::from("hello coders!!");
//     let s2=Strinf::from("let's GO !!");
//     let result=longest(&s1,&s2);
//     println!("The longest string is {}",result);
// }



// //lifetimes in structs
// struct ImportantExcerpt<'a>
// {
//     part:&'a str,
// }

// fn main()
// {
//     let novel=String::from("hello coders!!");
//     let first_sentence=novel.split('.').next().unwrap();
//     let excerpt=ImportantExcerpt{part:first_sentence};

//     println!("Excerpt:{}",excerpt.part);
// }



// //lifetime in methods
// struct ImportantExcerpt<'a>
// {
//     part:&'a str,
// }
// impl<'a> ImportantExcerpt<'a>
// {
//     fn announce_and_return_part(&self,announcement:&str)->&str
//     {
//         println!("Attention please:{}",announcement);
//         self.part
//     }
// }

// fn main()
// {
//     let novel=String::from("Hello coders!!");
//     let first_sentence=novel.split('.').next().unwrap();
//     let excerpt=ImportantExcerpt {part:first_sentence};

//     let announcement=excerpt.announce_and_return_part("Important excerpt!");
//     println!("{}",announcement)
// }



// //static lifetimes
// fn main()
// {
//     let s:&'static str="hello coders!!";
//     println!("{}",s);
// }


//multiple lifetimes
fn multiple_lifetimes<'a,'b>(x:&'a str,y:&'b str)->&'a str
{
    x
    
}
fn main()
{
    let s1=String::from("Rust");
    let s2=String::from("Programming");
    let result=multiple_lifetimes(&s1,&s2);
    println!("{}",result);

}