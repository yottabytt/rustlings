// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we instead create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!


fn main() {
    let vec0:Vec<i32> = Vec::new(); 
    /*
     when no code does type inference we need to mention type to create a 
     variable and leave it unused like above. let's look at another example 
     to just do that 
    */
    let vec3 = Vec::new();
    type_infer(vec3);
    /*
    an example of why vec0 was not causing any problem in the previous exercises
    */

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer take `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn type_infer(vec: Vec<i32>) {
    //do nothing
}
