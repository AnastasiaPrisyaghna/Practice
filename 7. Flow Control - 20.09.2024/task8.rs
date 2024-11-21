\fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue; // Fill in the first blank: Skip the rest of the loop when n != 66
        }
        
        break; // Fill in the second blank: Break the loop when n reaches 66
    }

    assert_eq!(n, 66);

    println!("Success!");
}

