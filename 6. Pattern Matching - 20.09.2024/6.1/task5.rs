// Заповнити пропуск
fn main() {
    let s = String::from("I like dogs");
    // Виділяємо нову пам'ять та зберігаємо змінений рядок там
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

