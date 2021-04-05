// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

/*
https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
straightforward syntax thing.
*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro![];
    my_macro!();
    my_macro!{};
}
