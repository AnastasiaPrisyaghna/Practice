// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();  // Це просто ідентифікатор типу `()`

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());  // Порівняння

    println!("Success!");
}

fn implicitly_ret_unit() -> () {  // Потрібно додати явне повернення типу `()`
    println!("I will return a ()");
    ()  // Явно повертаємо `()`
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
