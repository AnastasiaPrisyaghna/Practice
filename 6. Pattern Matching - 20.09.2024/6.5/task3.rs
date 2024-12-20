struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255);  // заповнюємо значення для Point
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);  // замінюємо __ на p.2
}
