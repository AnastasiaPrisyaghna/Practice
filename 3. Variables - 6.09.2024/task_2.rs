fn main() {
    let mut x: i32 = 1; // Використовуємо `mut` для того, щоб зробити змінну `x` змінною.
    x += 2; // Тепер можна змінювати значення змінної `x`.

    assert_eq!(x, 3); // Перевірка, чи дорівнює `x` 3.
    println!("Success!"); // Виведення успішного повідомлення.
}
