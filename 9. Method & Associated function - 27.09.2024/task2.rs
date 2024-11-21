#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {  // Використовуємо &self для того, щоб не брати власність
        println!("the current state is {}", self.color);
    }
}

fn main() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // Не забираємо власність у `light`
    light.show_state();
