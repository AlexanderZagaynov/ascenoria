#[cfg(test)]
mod tests {
    use crate::data_types::load_game_data;

    use super::super::{format_planet, generate_planet};

    #[test]
    fn generate_planets_smoke_test() {
        let (game_data, _registry) = load_game_data("assets/data").expect("load_game_data failed");
        let seeds = [1u64, 2, 3, 42, 1337, 9999];
        for seed in seeds {
            let planet = generate_planet(seed, &game_data).expect("expected some planet");
            let s = format_planet(&planet);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn orbital_placement_errors() {
        let (game_data, _registry) = load_game_data("assets/data").expect("load_game_data failed");

        let mut planet = generate_planet(42, &game_data).expect("expected some planet");
        if planet.orbital_slots == 0 {
            return;
        }

        for _ in 0..planet.orbital_slots {
            planet.place_orbital("some_project").expect("place_orbital");
        }

        let next = planet.place_orbital("some_project");
        assert!(next.is_err());
    }
}
