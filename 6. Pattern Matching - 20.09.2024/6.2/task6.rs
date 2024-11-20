// Fix the error
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // Use `.get` instead of indexing to avoid panic
    let _name1 = names.get(2).unwrap_or(&String::from("Default Name"));

    println!("Success!");
}
