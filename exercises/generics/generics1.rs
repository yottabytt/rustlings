// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

/*
just added the type as string slice
*/

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
