fn main() {
    let x: u32 = 5;   // змінили тип x на u32
    let mut y: u32 = 5;

    y = x;  // тепер x і y одного типу, немає конфлікту

    let z = 10;  // Тип z буде автоматично визначений як i32

    println!("Success!");
}
