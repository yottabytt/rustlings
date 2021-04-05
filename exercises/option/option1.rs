// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

/*
https://www.joshmcguigan.com/blog/array-initialization-rust/
*/

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Option::Some(99)); // both types of Some call works.

    let mut numbers: [Option<u16>; 5] = [None; 5]; // one among many ways 2 init
    for iter in 0..5 {
        let number_to_add: Option<u16> = {
            Some(((iter * 1235) + 2) / (4 * 16))
        };

        numbers[iter as usize] = number_to_add;
    }
}
