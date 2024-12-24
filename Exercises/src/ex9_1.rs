// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()//add to string here cause it is a string
}

pub fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}