//! ECS systems for the planet view.
//!
//! Contains input handling, button interactions, and cleanup systems.

use bevy::prelude::*;

use crate::{GalaxyPreview, main_menu::GameState, star::StarState};

use super::modal::PlanetInfoModalState;
use super::types::{PanelButton, PlanetThumbnail, PlanetView3D, PlanetViewRoot, colors};

/// Clean up all planet view entities when leaving the screen.
pub fn cleanup_planet_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<PlanetViewRoot>>,
    view_3d_query: Query<Entity, With<PlanetView3D>>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn();
    }
    for entity in &view_3d_query {
        commands.entity(entity).despawn();
    }
}

/// Handle keyboard navigation - ESC returns to star system, I shows info modal.
pub fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut modal_state: ResMut<PlanetInfoModalState>,
    star_state: Res<StarState>,
    galaxy_preview: Res<GalaxyPreview>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if modal_state.visible {
            modal_state.hide();
        } else {
            next_state.set(GameState::StarView);
        }
    }

    // Press 'I' to show planet info modal
    if keyboard.just_pressed(KeyCode::KeyI) && !modal_state.visible {
        let star_index = star_state.star_index;
        let planet_index = star_state.selected_planet.unwrap_or(0);

        if let Some(system) = galaxy_preview.galaxy.systems.get(star_index) {
            if let Some(planet) = system.planets.get(planet_index) {
                // Generate planet name like setup does
                let planet_name = format!("Planet {}", planet_index + 1);

                // Calculate some mock values for prosperity
                let prosperity = 1; // 1 per day base
                let days_to_growth = 20; // Placeholder
                let population = 0; // No population tracking yet
                let max_pop = planet.surface_slots as i32;

                modal_state.show(planet_name, prosperity, days_to_growth, population, max_pop);
            }
        }
    }
}

/// Handle panel buttons (back, toggle rotation, thumbnails).
pub fn panel_button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &PanelButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut thumbnail_query: Query<
        (&Interaction, &PlanetThumbnail, &mut BorderColor),
        (Changed<Interaction>, With<Button>, Without<PanelButton>),
    >,
    mut star_state: ResMut<StarState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Panel buttons
    for (interaction, mut bg_color, button) in &mut button_query {
        match *interaction {
            Interaction::Pressed => match button {
                PanelButton::Back => {
                    next_state.set(GameState::StarView);
                }
            },
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVERED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
            }
        }
    }

    // Planet thumbnail clicks
    for (interaction, thumbnail, mut border_color) in &mut thumbnail_query {
        if *interaction == Interaction::Pressed {
            star_state.selected_planet = Some(thumbnail.0);
            // Re-enter planet view to refresh
            next_state.set(GameState::PlanetView);
        } else if *interaction == Interaction::Hovered {
            *border_color = BorderColor::all(colors::THUMBNAIL_SELECTED.with_alpha(0.7));
        }
    }
}
