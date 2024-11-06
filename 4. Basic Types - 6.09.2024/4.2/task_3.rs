// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if t {  // Умова повинна бути true, щоб вивести "Success!"
        println!("Success!");
    }
}
