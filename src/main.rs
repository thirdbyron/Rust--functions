fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: u32) {
    println!("Hello from another function! This is a parameter - {x}")
}