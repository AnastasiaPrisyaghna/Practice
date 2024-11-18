fn main() {
    let s = "hello, world";
    greetings(s.to_string()); // Перетворення &str у String
}

fn greetings(s: String) {
    println!("{}", s)
}
