// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

/*

had to read the following chapters to get this done
https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
https://doc.rust-lang.org/book/ch13-01-closures.html
https://doc.rust-lang.org/book/ch16-00-concurrency.html

Arc is especially in 
https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct

*/

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    //println!("numbers len is {}", numbers.len());
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        //println!("child_numbers len is {}", child_numbers.len());
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
