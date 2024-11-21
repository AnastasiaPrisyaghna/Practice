fn main() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names { // Borrow the elements to avoid moving
        // Do something with name...
    }

    println!("{:?}", names); // Now we can use `names` after borrowing

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy, so there is no move here
    for n in numbers {
        // Do something with n...
    }
    
    println!("{:?}", numbers); // `numbers` is still valid because they are Copy
}
