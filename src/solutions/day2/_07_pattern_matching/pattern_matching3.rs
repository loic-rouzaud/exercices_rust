pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Windy(u32), // Wind speed in km/h
}

pub fn get_activity(weather: &Weather) -> String {
    match weather {
        Weather::Sunny => String::from("Go to the beach"),
        Weather::Cloudy => String::from("Visit a museum"),
        Weather::Rainy => String::from("Read a book at home"),
        Weather::Snowy => String::from("Build a snowman"),
        Weather::Windy(speed) => {
            if *speed < 20 {
                String::from("Go fly a kite")
            } else if *speed <= 40 {
                String::from("Stay in a park")
            } else {
                String::from("Stay inside")
            }
        }
    }
}

fn main() {
    println!("Sunny: {}", get_activity(&Weather::Sunny));
    println!("Cloudy: {}", get_activity(&Weather::Cloudy));
    println!("Rainy: {}", get_activity(&Weather::Rainy));
    println!("Snowy: {}", get_activity(&Weather::Snowy));
    println!("Windy (15km/h): {}", get_activity(&Weather::Windy(15)));
    println!("Windy (30km/h): {}", get_activity(&Weather::Windy(30)));
    println!("Windy (50km/h): {}", get_activity(&Weather::Windy(50)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunny() {
        assert_eq!(get_activity(&Weather::Sunny), "Go to the beach");
    }

    #[test]
    fn test_cloudy() {
        assert_eq!(get_activity(&Weather::Cloudy), "Visit a museum");
    }

    #[test]
    fn test_rainy() {
        assert_eq!(get_activity(&Weather::Rainy), "Read a book at home");
    }

    #[test]
    fn test_snowy() {
        assert_eq!(get_activity(&Weather::Snowy), "Build a snowman");
    }

    #[test]
    fn test_windy() {
        assert_eq!(get_activity(&Weather::Windy(10)), "Go fly a kite");
        assert_eq!(get_activity(&Weather::Windy(30)), "Stay in a park");
        assert_eq!(get_activity(&Weather::Windy(41)), "Stay inside");
    }
}
