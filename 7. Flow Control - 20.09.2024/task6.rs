\fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 { // Fill in the first blank: `<= 10`
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1; // Fill in the second blank: `n += 1` to increment `n`
    }

    println!("n reached {}, so loop is over", n);
}

