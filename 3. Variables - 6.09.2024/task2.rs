fn main() {
    let mut x = 1;  // Використовуємо `mut`, щоб змінну `x` можна було змінювати
    x += 2;  // Тепер можна змінити значення `x`, додавши 2 до нього
    
    assert_eq!(x, 3);  // Перевіряємо, що значення `x` після операції дорівнює 3
    println!("Success!");  // Якщо все вірно, виводимо повідомлення про успіх
}