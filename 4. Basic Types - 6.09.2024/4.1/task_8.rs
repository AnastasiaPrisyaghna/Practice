fn main() {
    // Перший спосіб: використання epsilon для точності
    assert!((0.1 + 0.2 - 0.3).abs() < f64::EPSILON);

    // Другий спосіб: використання round()
    assert!((0.1 + 0.2).round() == 0.3);

    println!("Success!");
}
