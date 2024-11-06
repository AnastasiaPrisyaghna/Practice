fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y); // Доступ до `y` в межах блоку
    }
    // println!("The value of x is {} and value of y is {}", x, y); // Цей рядок треба прибрати або змінити
    println!("The value of x is {}", x); // Потрібно використовувати лише `x`, бо `y` вийшла з області видимості
}
