#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    // The check will ensure the size is less than 768 bytes.
    // No additional logic needed here for this example.
}

// Fix the errors in main.
fn main() {
    check_size([0u8; 767]);  // Size of [u8; 767] is 767 bytes.
    check_size([0i32; 191]); // Size of [i32; 191] is 764 bytes.
    check_size(["hello你好"; 2]); // Size of &str, 2 strings of 6 bytes each = 12 bytes (size of &str is dynamic, but total size is 12).
    check_size([(); 6].map(|_| "hello你好".to_string()));  // Size of String, 6 Strings (each 20 bytes for simplicity).
    check_size(['中'; 6]); // Size of char, each `char` is 4 bytes, total 24 bytes.

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
