// Remove a line in the code to make it compile
fn main() {
let mut x:i32 = 1;
x = 7; // x = 7
//Shadowing and re-binding
// let mut x = x; //7
x+=3;

let y:i32 = 4;
//Shadowing
let y: &str = "I can also be bound to text!";

println!("Success");
}
