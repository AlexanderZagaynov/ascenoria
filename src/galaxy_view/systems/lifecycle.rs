use bevy::prelude::*;

use crate::galaxy_view::types::GalaxyViewRoot;

/// Clean up all galaxy map entities when leaving the screen.
pub fn cleanup_galaxy_view(mut commands: Commands, query: Query<Entity, With<GalaxyViewRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
