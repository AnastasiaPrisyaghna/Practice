use std::ops::Add;  // Імпортуємо трейт Add з бібліотеки std

// Перший спосіб: Використання трейту Add
fn sum_1<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,  // Переконуємось, що T підтримує додавання і результат також має тип T
{
    x + y
}

// Другий спосіб: Використання власного трейту для додавання
trait MyAdd {
    fn add(self, other: Self) -> Self;
}

impl MyAdd for i32 {
    fn add(self, other: i32) -> i32 {
        self + other
    }
}

fn sum_2<T: MyAdd>(x: T, y: T) -> T {
    x.add(y)
}

fn main() {
    assert_eq!(sum_1(1, 2), 3);  // Перший спосіб
    assert_eq!(sum_2(1, 2), 3);  // Другий спосіб

    println!("Успіх!");
}
