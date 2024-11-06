fn main() {
    let (mut x, y) = (1, 2);  // Додаємо mut для змінної x
    x += 2; // Тепер можна змінити x

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
