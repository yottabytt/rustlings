// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)


/*
like Cish langs, macro should be defined before they can be used.
*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}

