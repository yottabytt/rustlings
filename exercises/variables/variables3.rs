// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

/* 
variables by default in rust are immutable. so one needs to give mut keyword
to make it mutable
*/


fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
