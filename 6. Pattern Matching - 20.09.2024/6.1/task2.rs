// Solution 1: Dereferencing Box<str> to &str
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s); // Передаємо посилання
}

fn greetings(s: &str) {
    println!("{}", s);
}

// Solution 2: Modifying greetings to accept Box<str>
fn main_alt() {
    let s: Box<str> = "hello, world".into();
    greetings_alt(s); // Передаємо Box<str>
}

fn greetings_alt(s: Box<str>) {
    println!("{}", s);
}
