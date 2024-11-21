trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

// IMPLEMENT this function.
fn hatch_a_bird(bird_type: i32) -> Box<dyn Bird> {
    if bird_type == 1 {
        Box::new(Swan)
    } else {
        Box::new(Duck)
    }
}

fn main() {
    // FILL in the blank.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim(); // This will cause an error, as bird is of type Bird, not Duck
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly(); // This will cause an error, as bird is of type Bird, not Swan
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}
