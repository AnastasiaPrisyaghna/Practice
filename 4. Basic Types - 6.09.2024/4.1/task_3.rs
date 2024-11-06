fn main() {
    let x = 5; // Тип x буде inferred як i32
    assert_eq!("i32", type_of(&x));  // Використовуємо правильний тип для порівняння

    println!("Success!");
}

// Функція для отримання типу змінної
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
