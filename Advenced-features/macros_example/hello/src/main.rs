use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    println!("{}", stringify!(Pancakes));
    Pancakes::hello_macro();
}
