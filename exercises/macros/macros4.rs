// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // ; is the char that needs to be added in between arms
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
