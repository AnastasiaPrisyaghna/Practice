fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Змінили на 12 для того, щоб не виникала помилка
    }

    assert_eq!(x, 5); // Залишаємо тут 5, оскільки знову використовуємо зовнішню змінну x

    let x = 42;
    println!("{}", x); // Виводить "42".
}