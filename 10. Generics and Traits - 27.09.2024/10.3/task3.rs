use std::ops::Mul;

// Implement the multiply function using the Mul trait.
fn multiply<T>(a: T, b: T) -> T
where
    T: Mul<Output = T>, // Ensure T supports multiplication
{
    a * b // Use the `*` operator to multiply a and b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8)); // Works with u8
    assert_eq!(5.0, multiply(1.0, 5.0)); // Works with f64

    println!("Success!");
}
