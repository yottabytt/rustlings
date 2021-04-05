// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// Ownership, references, and borrowing

fn main() {

    /*
    let vec0 = Vec::new();
    let mut vec1 = fill_vec1(vec0.clone());
    */

    /*
    let vec0 = Vec::new();
    let mut vec1 = fill_vec2(&vec0);
    */

    
    let mut vec0 = Vec::new();
    fill_vec3(&mut vec0);
    let mut vec1 = Vec::new();
    

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// move
fn fill_vec1(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// borrow
fn fill_vec2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// mutable borrow
fn fill_vec3(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
