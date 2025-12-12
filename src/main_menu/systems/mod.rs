use bevy::prelude::*;

use super::components::MainMenuRoot;

mod interactions;
mod layout;

pub use interactions::{button_system, menu_action_system};
pub use layout::setup_main_menu;

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
