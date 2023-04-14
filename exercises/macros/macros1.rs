// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.



// Metaprogramming is useful for reducing the amount of code you have to write and maintain,
// All of these macros expand to produce more code than the code you’ve written manually.


// declarative macro.

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
