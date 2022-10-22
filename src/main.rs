fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: u32) {
    let y = function_return_number(8);
    println!("Hello from another function! This is a parameter - {}. And this returned by one more function - {}", x, y)
}

fn function_return_number(y: i8) -> i8 {
    if y != 0 {
        y
    } else {
        y + 1
    }
}
