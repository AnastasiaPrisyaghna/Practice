struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Використовуємо &self для доступу до екземпляра
    pub fn show_state(&self)  { 
        println!("the current state is {}", self.color);
    }

    // Використовуємо &mut self для зміни стану
    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

fn main() {
    let mut light = TrafficLight {
        color: "red".to_string(),
    };

    light.show_state(); // Виводимо поточний стан
    light.change_state(); // Змінюємо стан на зелене
    light.show_state(); // Виводимо новий стан

    println!("Success!");
}
