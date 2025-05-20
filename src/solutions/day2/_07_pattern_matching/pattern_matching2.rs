enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn get_action(light: TrafficLight) -> String {
    match light {
        TrafficLight::Red => String::from("Arrêter"),
        TrafficLight::Yellow => String::from("Ralentir"),
        TrafficLight::Green => String::from("Avancer"),
    }
}

fn main() {
    println!("Rouge: {}", get_action(TrafficLight::Red));
    println!("Jaune: {}", get_action(TrafficLight::Yellow));
    println!("Vert: {}", get_action(TrafficLight::Green));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_light() {
        assert_eq!(get_action(TrafficLight::Red), "Arrêter");
    }

    #[test]
    fn test_yellow_light() {
        assert_eq!(get_action(TrafficLight::Yellow), "Ralentir");
    }

    #[test]
    fn test_green_light() {
        assert_eq!(get_action(TrafficLight::Green), "Avancer");
    }
}
