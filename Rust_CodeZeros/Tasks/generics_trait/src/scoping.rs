pub fn main()
{
    let x=100;
    println!("x is {}",x);
    {
        // let x=15;
        let y=25;
        println!("x= {}, y={} ",x,y);

    }
    println!("x={}",x);
    // println!("x= {}, y={} ",x,y);


    let s=String::from("Hello");
    {
        let s1=s;
        println!("s1:{}",s1);
    }
    // println!("s:{}",s); // it is no longer ownership has been transfer


    let x=5;
    let x=x+1; // shadowing the x // 6
    {
        let x=x*2;
        println!("x={}",x); //12
    }
    println!("x={}",x);//6


    let mut x=10;
    {
        x+=5;
        println!("Inner x:{}",x); //10
    }
    x *= 2;
    println!("Outer x:{}",x); //30

}