//! ECS systems for the star system view.
//!
//! Contains interaction, camera control, and cleanup systems.

use bevy::prelude::*;

use crate::main_menu::GameState;

use super::types::{PlanetMarker, StarRoot, StarState};

/// Clean up all star system view entities when leaving the screen.
pub fn cleanup_star(mut commands: Commands, query: Query<Entity, With<StarRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handle hovering over planets.
pub fn planet_hover_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<StarRoot>>,
    mut planet_query: Query<(&PlanetMarker, &Transform, &mut Sprite)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<StarState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mut hovered_planet: Option<usize> = None;

    for (marker, transform, mut sprite) in &mut planet_query {
        let planet_pos = transform.translation.truncate();
        let size = sprite.custom_size.unwrap_or(Vec2::splat(30.0)).x;
        let distance = world_position.distance(planet_pos);

        if distance < size / 2.0 + 10.0 {
            hovered_planet = Some(marker.planet_index);
            // Highlight on hover
            sprite.color = sprite.color.with_alpha(1.0);
        } else {
            sprite.color = sprite.color.with_alpha(0.9);
        }
    }

    // Click to select planet and navigate to planet view
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(idx) = hovered_planet {
            state.selected_planet = Some(idx);
            info!("Selected planet {}, opening planet view", idx);
            next_state.set(GameState::PlanetView);
        }
    }
}

/// Handle panel button interactions.
pub fn panel_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(bg_color.0.with_alpha(1.0));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(bg_color.0.with_alpha(0.95));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(bg_color.0.with_alpha(0.85));
            }
        }
    }
}

/// Handle camera/view controls.
pub fn camera_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, With<StarRoot>)>,
    mut state: ResMut<StarState>,
) {
    let mut delta = Vec2::ZERO;
    let rotation_speed = 2.0;

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        delta.x -= rotation_speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        delta.x += rotation_speed;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        delta.y += rotation_speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        delta.y -= rotation_speed;
    }

    if delta != Vec2::ZERO {
        for mut transform in &mut camera_query {
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }

    // Zoom controls
    if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        state.zoom = (state.zoom + 0.02).min(2.0);
    }
    if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        state.zoom = (state.zoom - 0.02).max(0.5);
    }
}

/// Handle keyboard navigation (ESC to return to galaxy map).
pub fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to galaxy map...");
        next_state.set(GameState::GalaxyView);
    }
}
