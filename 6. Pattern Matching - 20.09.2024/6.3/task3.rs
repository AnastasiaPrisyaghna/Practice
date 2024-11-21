fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4]; // від індексу 1 до 4 (включаючи 1, але не включаючи 4)
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
