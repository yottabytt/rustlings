// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

/*
remember that my_macro! in assert_eq! does a replacement with macro code
so it should be scoped using {} inside the arm and should have a ruby return 
statement
*/

macro_rules! my_macro {
    ($val:expr) => {
        {
            let mut s = String::from("Hello ");
            s.push_str($val);
            s
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
