use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // Виправлено на 4, бо Rust використовує UTF-8 для символів

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); // Для UTF-8 кодування також буде 4 байти

    println!("Success!");
}
