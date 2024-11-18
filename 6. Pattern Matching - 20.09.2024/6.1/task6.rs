// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Додаємо ссилку на s2, щоб не забирати власність
    assert_eq!(s3, "hello,world!");
    println!("{}", s1); // Виводиться оригінальний s1, тому що він не був переміщений
}
