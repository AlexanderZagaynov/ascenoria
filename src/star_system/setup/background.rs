use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::star_system::types::StarSystemRoot;

/// Spawn background stars for atmosphere.
pub fn spawn_background_stars(commands: &mut Commands) {
    let mut rng = StdRng::seed_from_u64(999);

    for _ in 0..80 {
        let x = Rng::gen_range(&mut rng, -500.0..500.0);
        let y = Rng::gen_range(&mut rng, -350.0..350.0);
        let brightness = Rng::gen_range(&mut rng, 0.15..0.5);
        let size = Rng::gen_range(&mut rng, 1.0..2.5);

        commands.spawn((
            Sprite {
                color: Color::srgba(brightness, brightness, brightness * 1.1, 0.7),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -5.0)),
            StarSystemRoot,
        ));
    }
}
