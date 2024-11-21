#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Реалізація методу `color` для `TrafficLightColor`
impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}
