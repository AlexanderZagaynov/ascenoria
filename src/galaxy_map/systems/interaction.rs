use bevy::{prelude::*, window::PrimaryWindow};

use crate::main_menu::GameState;
use crate::galaxy_map::colors;
use crate::galaxy_map::types::{GalaxyMapRoot, GalaxyMapState, PanelButton, SelectionIndicator, StarMarker};

/// Handle clicking on stars to select/enter systems.
pub fn star_click_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), (With<GalaxyMapRoot>, With<Camera3d>)>,
    star_query: Query<(&StarMarker, &GlobalTransform)>,
    mut selection_query: Query<&mut Transform, (With<SelectionIndicator>, Without<StarMarker>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut map_state: ResMut<GalaxyMapState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut star_system_state: ResMut<crate::star_system::StarSystemState>,
    galaxy_preview: Res<crate::GalaxyPreview>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    // If left-click became a drag (moved beyond threshold), don't select a star
    if map_state.left_is_dragging {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Cast a ray from the camera through the cursor position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Find the closest star that the ray hits
    let mut closest_star: Option<(usize, f32, Vec3)> = None;
    let click_radius = 0.5; // How close the ray needs to pass to a star

    for (marker, star_transform) in &star_query {
        let star_pos = star_transform.translation();

        // Calculate distance from ray to star
        let to_star = star_pos - ray.origin;
        let proj_length = to_star.dot(*ray.direction);

        if proj_length < 0.0 {
            continue; // Star is behind the camera
        }

        let closest_point = ray.origin + *ray.direction * proj_length;
        let distance = (star_pos - closest_point).length();

        if distance < click_radius {
            if closest_star.is_none() || proj_length < closest_star.unwrap().1 {
                closest_star = Some((marker.system_index, proj_length, star_pos));
            }
        }
    }

    // Handle click
    if let Some((idx, _, star_pos)) = closest_star {
        let system_name = galaxy_preview
            .galaxy
            .systems
            .get(idx)
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");

        if map_state.selected_system == Some(idx) {
            // Double-click on same star - enter system view
            star_system_state.system_index = idx;
            star_system_state.selected_planet = None;
            next_state.set(GameState::StarSystem);
            info!("Entering system {} ({})", idx, system_name);
        } else {
            map_state.selected_system = Some(idx);

            // Move selection indicator to the selected star
            if let Ok(mut selection_transform) = selection_query.single_mut() {
                selection_transform.translation = star_pos;
            }

            info!("Selected system {} ({})", idx, system_name);
        }
    }
}

/// Handle panel button interactions.
pub fn panel_button_system(
    mut interaction_query: Query<
        (&Interaction, &PanelButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    map_state: Res<GalaxyMapState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut star_system_state: ResMut<crate::star_system::StarSystemState>,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Panel button pressed: {:?}", button);
                *bg_color = BackgroundColor(colors::PANEL_DARK.with_alpha(1.0));

                // Handle button actions
                match button {
                    PanelButton::Planets => {
                        if let Some(system_idx) = map_state.selected_system {
                            star_system_state.system_index = system_idx;
                            star_system_state.selected_planet = None;
                            next_state.set(GameState::StarSystem);
                            info!("Entering system {} via Planets button", system_idx);
                        }
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.35, 0.38, 0.42));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::PANEL_DARK);
            }
        }
    }
}
