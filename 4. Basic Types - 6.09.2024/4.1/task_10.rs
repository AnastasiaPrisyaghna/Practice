use std::ops::{Range, RangeInclusive};

fn main() {
    // Заповнюємо пропуски: для Range ми використовуємо два числа, для RangeInclusive – включно.
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
