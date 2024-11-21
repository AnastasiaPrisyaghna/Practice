// Define the struct `Point` with generics to work with any type `T`
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // Create an instance of Point with integer values
    let integer = Point { x: 5, y: 10 };

    // Create an instance of Point with floating-point values
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
