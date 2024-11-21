fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}