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


//multiple  Traits
trait Walk
{
    fn walk(&self);

}
trait Talk
{
    fn talk(&self);

}

struct Human;

impl Walk for Human
{
    fn walk(&self)
    {
        println!("I am walking")
    }
}


impl Talk for Human
{
    fn talk(&self)
    {
        println!("I am Talking")
    }
}


//Trait Objects
trait Animal
{
    fn make_sound(&self);

}

struct Dog1;
struct Cat1;

impl Animal for Dog1
{
    fn make_sound(&self)
    {
        println!("Woof! Woof!")
    }
}

impl Animal for Cat1
{
    fn make_sound(&self)
    {
        println!(" Meow!! Meow!!")
    }
}

//Deriving Traits
#[derive(Debug,Clone,PartialEq)]
struct Point1
{
    x:i32,
    y:i32,
}

//Associated Types In Traits 
trait Container
{
    type Item;
    fn contains(&self,item:&Self::Item)->bool;
}

struct Bag
{
    items:Vec<String>,
}

impl Container for Bag
{
    type Item=String;

    fn contains(&self,item: &Self::Item)->bool
    {
        self.items.contains(item)
    }
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

    let human=Human;
    human.walk();
    human.talk();


    let animals:Vec<Box<dyn Animal>>=vec![Box::new(Dog1),Box::new(Cat1)];
    for animal in animals
    {
        animal.make_sound();
    }

    let p1=Point1{x:1,y:2};
    let p2=p1.clone();

    println!("{:?}",p1);
    println!("Equal:{}",p1==p2);


    let bag=Bag
    {
        items:vec!["apple".to_string(),"banana".to_string()],
    };
    
    println!("Contains Apple:{}",bag.contains(&"apple".to_string()));
    

}