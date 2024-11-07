fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = &x;  // Borrow x instead of moving it
    println!("{}, {}", x, y);  // Both x and y can be used
}
