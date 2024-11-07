fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    loop {
        // Тут можна додати будь-який код, якщо потрібно.
    }
}
