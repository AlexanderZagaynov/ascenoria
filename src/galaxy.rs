use rand::{Rng, RngCore, SeedableRng, rngs::StdRng};

use crate::{
    data::GameData,
    planet::{GeneratedPlanet, format_planet, generate_planet},
};

/// A star system with a deterministic list of generated planets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarSystem {
    /// Human-readable system name.
    pub name: String,
    /// Generated planets within the system.
    pub planets: Vec<GeneratedPlanet>,
}

/// Simple seeded galaxy containing multiple systems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Galaxy {
    /// All generated star systems.
    pub systems: Vec<StarSystem>,
}

/// Generate a galaxy using the provided seed and planet data.
pub fn generate_galaxy(
    seed: u64,
    data: &GameData,
    system_range: std::ops::RangeInclusive<usize>,
    planet_range: std::ops::RangeInclusive<usize>,
) -> Galaxy {
    let mut rng = StdRng::seed_from_u64(seed);
    let system_count = clamp_range(&mut rng, system_range.clone());

    let mut systems = Vec::with_capacity(system_count);
    for idx in 0..system_count {
        let planets_count = clamp_range(&mut rng, planet_range.clone());
        let mut planets = Vec::with_capacity(planets_count);

        for _ in 0..planets_count {
            // Derive a planet seed from the galaxy RNG to keep determinism.
            let planet_seed = rng.next_u64();
            if let Some(planet) = generate_planet(planet_seed, data) {
                planets.push(planet);
            }
        }

        systems.push(StarSystem {
            name: format!("System-{idx}"),
            planets,
        });
    }

    Galaxy { systems }
}

fn clamp_range(rng: &mut StdRng, range: std::ops::RangeInclusive<usize>) -> usize {
    let min = *range.start();
    let max = *range.end();
    if min >= max {
        return min;
    }
    rng.gen_range(min..=max)
}

/// Render a galaxy with all systems and planets for debug logging.
pub fn format_galaxy(galaxy: &Galaxy) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Galaxy with {} systems", galaxy.systems.len()));

    for (sys_idx, system) in galaxy.systems.iter().enumerate() {
        lines.push(format!(
            "\nSystem {sys_idx}: {} ({} planets)",
            system.name,
            system.planets.len()
        ));
        for (planet_idx, planet) in system.planets.iter().enumerate() {
            lines.push(format!("  Planet {planet_idx}:"));
            for line in format_planet(planet).lines() {
                lines.push(format!("    {line}"));
            }
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_game_data;
    use std::path::PathBuf;

    #[test]
    fn deterministic_generation_with_seed() {
        let (data, _) =
            load_game_data(PathBuf::from("assets/data")).expect("Game data should load");

        let g1 = generate_galaxy(7, &data, 2..=3, 1..=2);
        let g2 = generate_galaxy(7, &data, 2..=3, 1..=2);
        assert_eq!(g1, g2, "same seed should yield identical galaxy");

        let g3 = generate_galaxy(8, &data, 2..=3, 1..=2);
        assert_ne!(g1, g3, "different seed should yield different galaxy");
    }

    #[test]
    fn formats_galaxy() {
        let (data, _) =
            load_game_data(PathBuf::from("assets/data")).expect("Game data should load");
        let galaxy = generate_galaxy(7, &data, 1..=1, 1..=1);
        let formatted = format_galaxy(&galaxy);

        assert!(formatted.contains("Galaxy with 1 systems"));
        assert!(formatted.contains("System 0"));
        assert!(formatted.contains("Planet 0"));
    }
}
