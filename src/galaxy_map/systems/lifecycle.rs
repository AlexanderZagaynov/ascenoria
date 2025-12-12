use bevy::prelude::*;

use crate::galaxy_map::types::GalaxyMapRoot;

/// Clean up all galaxy map entities when leaving the screen.
pub fn cleanup_galaxy_map(mut commands: Commands, query: Query<Entity, With<GalaxyMapRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
