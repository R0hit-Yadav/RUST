//Modules 
// is use for organize code into smaller units control the scope and visibility of items like functions ,
//structs and constants for mantainable and scalling applictions
mod module; 
mod my_module
{
    pub fn greet()
    {
        println!("Hello Coders")
    }
}

//for nested 
mod outer
{
    pub mod inner{
        pub fn say_hello()
        {
            println!("hello from inner module!")
        }
    }
}
//visibility
mod my_module2
{
    pub fn public_function()
    {
        println!("This is public function")
    }
    fn private_function()
    {
        println!("This is private function")
    }
}

//re-export 
mod my_module3
{
    pub mod inner{
       pub fn export()
        {
            println!("This is exported function")

        }
    }
    pub use inner::export; //exporting
}

//use in nested 
mod outer2
{
    pub mod inner2
    {
        pub fn greet3()
        {
            println!("Use of use");
        }
    }
   
}
use outer2::inner2::greet3;
fn main() {
    my_module::greet();

    outer::inner::say_hello();

    my_module2::public_function();
    // my_module2::private_function(); // not accessible 

    //from another file
    module::greet2();

    my_module3::export();

    greet3();
    
}

