struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays = [
        Array {
            data: [1, 2, 3], // `i32` array with size 3
        },
        Array {
            data: [1.0, 2.0, 3.0], // `f32` array with size 3
        },
        Array {
            data: [1, 2], // `i32` array with size 2
        },
    ];

    println!("Success!");
}
