// Make it work
fn main() {
    let f = true;
    let t = true && true;  // t буде true, оскільки true && true = true
    assert_eq!(t, f);  // Тепер t і f будуть рівні

    println!("Success!");
}
