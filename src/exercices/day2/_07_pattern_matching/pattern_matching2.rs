pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn get_action(light: TrafficLight) -> String {
    // TODO(): Implémenter une fonction qui retourne l'action à effectuer
    // selon la couleur du feu de circulation:
    // - Si le feu est rouge: retourner "Arrêter"
    // - Si le feu est jaune: retourner "Ralentir"
    // - Si le feu est vert: retourner "Avancer"
}

fn main() {
    // pour tester votre fonction
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
