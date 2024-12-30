//static methods and object methods in Rust 


// //static methods
// struct Rectangle
// {
//     width:u32,
//     height:u32,
// }
// impl Rectangle
// {
//     fn square(size:u32)->Rectangle
//     {
//         Rectangle{width:size,height:size}
//     }
// }

// fn main()
// {
//     let square=Rectangle::square(5);
//     println!("Square: width={},height={}",square.width,square.height);
// }




// //object methods
// struct Rectangle
// {
//     width:u32,
//     height:u32,
// }
// impl Rectangle
// {
//     fn area(&self)->u32
//     {
//         self.width*self.height
//     }
// }

// fn main()
// {
//     let rect=Rectangle
//     {
//         width:10,
//         height:20,
//     };
//     let area=rect.area();
//     println!("Area: {}",area);
// }



// combining static and object methods
struct Circle
{
    raduis:f64,
}
impl Circle
{
    //static method
    fn new(raduis:f64)->Circle
    {
        Circle{raduis}
    }

    //object method
    fn area(&self)->f64
    {
        std::f64::consts::PI* self.raduis * self.raduis
 
    }
}

fn main()
{
    //using static create new circle
    let circle=Circle::new(10.0);

    //using object calculate
    let area=circle.area();
    println!("Circle Area:{:.2}",area);

}