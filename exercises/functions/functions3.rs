// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

/*
Rust does not have a notion of optional function arguments or
variadic functions
*/

fn main() {
    call_me(12);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
