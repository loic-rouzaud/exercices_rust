pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Windy(u32), // Wind speed in km/h
}

pub fn get_activity(weather: &Weather) -> String {
    // TODO(): Implémenter une fonction qui suggère une activité adaptée à la météo:
    // - Si c'est ensoleillé (Sunny): retourner "Go to the beach"
    // - Si c'est nuageux (Cloudy): retourner "Visit a museum"
    // - Si c'est pluvieux (Rainy): retourner "Read a book at home"
    // - Si c'est neigeux (Snowy): retourner "Build a snowman"
    // - Si c'est venteux (Windy):
    //   * Si la vitesse du vent est inférieure à 20 km/h: retourner "Go fly a kite"
    //   * Si la vitesse est entre 20 et 40 km/h: retourner "Stay in a park"
    //   * Si la vitesse est supérieure à 40 km/h: retourner "Stay inside"
}

fn main() {
    // pour tester votre fonction
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
