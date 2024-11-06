// Make it work
fn main() {
    let c1 = "中";
    print_char(c1);
}

fn print_char(c: &str) {
    for ch in c.chars() {
        println!("{}", ch);
    }
}
