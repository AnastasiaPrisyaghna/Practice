fn main() {
    let (mut x, mut y); // Оголошуємо змінні x і y як мутабельні для зміни їх значень
    (x, ..) = (3, 4);    // Деструктуруємо кортеж (3, 4), беремо значення для x
    .. = [1, 2];         // Слід за допомогою шаблону деструктурувати масив

    assert_eq!([x, y], [3, 2]); // Порівнюємо [x, y] з [3, 2]

    println!("Success!"); // Виводимо повідомлення
}
