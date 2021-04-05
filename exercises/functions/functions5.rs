// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// w/ ; is a statement
// w/o is an expression
// for functions with return vals we can have a return statement or a expr

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num; 
    // num * num also works
}
