// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

/*
interesting to know about self and Self
https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self
*/

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        let mut s = self;
        s.push_str("Bar");
        return s;
    }
}

/*
this also works :)

impl AppendBar for String {
    fn append_bar(self: String) -> String {
        let mut s = self;
        s.push_str("Bar");
        return s;
    }
}
*/

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
