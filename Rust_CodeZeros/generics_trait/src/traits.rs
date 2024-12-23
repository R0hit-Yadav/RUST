trait Speak
{
    fn speak(&self);
}
struct Dog;
struct Cat;

impl Speak for Cat
{
    fn speak(&self)
    {
        println!("Woof! Woof!")
    }
}

impl Speak for Dog
{
    fn speak(&self)
    {
        println!("Meow!!")
    }
}


//Trait Bounds
trait Addable
{
    fn add(&self,other:&Self)->Self;
}

struct Point
{
    x:i32,
    y:i32,
}

impl Addable for Point
{
    fn add(&self,other:&Self)->Self
    {
        Point
        {
            x:self.x+other.x,
            y:self.y+other.y,
        }
    }
}

fn add_points<T:Addable>(a:T,b:T)-> T
{
    a.add(&b)
}




pub fn traits()
{

    let dog =Dog;
    let cat=Cat;
    dog.speak();
    cat.speak();

    let p1=Point{x:1,y:2};
    let p2=Point{x:3,y:4};
    let result=add_points(p1,p2);

    println!("Result:({},{})",result.x,result.y);

}