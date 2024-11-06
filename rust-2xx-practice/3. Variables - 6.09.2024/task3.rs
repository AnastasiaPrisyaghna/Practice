fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // Другий виклик print! видалено, оскільки y вже не існує поза блоком
    println!("The value of x is {}", x);
}
