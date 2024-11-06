// Modify `4` in assert to make it work
use std::mem::size_of_val;

fn main() {
    let unit: () = ();  // Тип `()` займає 0 байт
    assert!(size_of_val(&unit) == 0);  // Використовуємо 0 замість 4, оскільки `()` займає 0 байт

    println!("Success!");
}
