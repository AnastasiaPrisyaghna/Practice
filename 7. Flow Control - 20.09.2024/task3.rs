fn main() {
    for n in 1..100 { // Use 1..100 to exclude 100 from the loop
        if n == 100 {
            panic!("NEVER LET THIS RUN");
        }
    }

    println!("Success!");
}
