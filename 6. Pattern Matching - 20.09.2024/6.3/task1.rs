// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // Use slice reference &[i32]

    let s2: &str = "hello, world"; // Use &str directly

    println!("Success!");
}
