//! ECS systems for the galaxy map.
//!
//! Contains the core gameplay systems: camera rotation, star selection,
//! panel button handling, and turn control.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::main_menu::GameState;

use super::colors;
use super::modal::{InfoModalState, ModalAction, ModalButton, ModalIcon};
use super::types::{GalaxyMapRoot, GalaxyMapState, PanelButton, SelectionIndicator, StarMarker};

/// Clean up all galaxy map entities when leaving the screen.
pub fn cleanup_galaxy_map(mut commands: Commands, query: Query<Entity, With<GalaxyMapRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handle galaxy rotation via mouse drag and camera orbit.
pub fn galaxy_rotation_system(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut map_state: ResMut<GalaxyMapState>,
    mut camera_query: Query<&mut Transform, (With<GalaxyMapRoot>, With<Camera3d>)>,
    time: Res<Time>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let cursor_pos = window.cursor_position();

    // Drag threshold in pixels - movement beyond this is considered a drag, not a click
    const DRAG_THRESHOLD: f32 = 5.0;

    // Start dragging with right mouse button or middle mouse button
    if buttons.just_pressed(MouseButton::Right) || buttons.just_pressed(MouseButton::Middle) {
        map_state.is_dragging = true;
        if let Some(pos) = cursor_pos {
            map_state.last_mouse_pos = pos;
        }
    }

    if buttons.just_released(MouseButton::Right) || buttons.just_released(MouseButton::Middle) {
        map_state.is_dragging = false;
    }

    // Handle left-click drag (for rotating on empty space)
    if buttons.just_pressed(MouseButton::Left) {
        map_state.left_mouse_down = true;
        map_state.left_is_dragging = false;
        if let Some(pos) = cursor_pos {
            map_state.left_click_start_pos = pos;
            map_state.last_mouse_pos = pos;
        }
    }

    if buttons.just_released(MouseButton::Left) {
        map_state.left_mouse_down = false;
        map_state.left_is_dragging = false;
    }

    // Check if left-click has moved enough to be considered a drag
    if map_state.left_mouse_down && !map_state.left_is_dragging {
        if let Some(pos) = cursor_pos {
            let distance_from_start = (pos - map_state.left_click_start_pos).length();
            if distance_from_start > DRAG_THRESHOLD {
                map_state.left_is_dragging = true;
            }
        }
    }

    // Rotate while dragging with mouse (right/middle click, or left-click drag)
    let is_any_drag = map_state.is_dragging || map_state.left_is_dragging;
    if is_any_drag {
        if let Some(pos) = cursor_pos {
            let delta = pos - map_state.last_mouse_pos;
            map_state.last_mouse_pos = pos;

            // Horizontal drag rotates around Y axis
            map_state.rotation_y += delta.x * 0.01;
            // Vertical drag rotates around X axis (clamped to avoid gimbal lock)
            map_state.rotation_x = (map_state.rotation_x - delta.y * 0.01).clamp(-1.2, 1.2);
        }
    }

    // Keyboard rotation controls
    let rotation_speed = 2.0 * time.delta_secs();

    // Arrow keys and WASD for rotation
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        map_state.rotation_y -= rotation_speed;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        map_state.rotation_y += rotation_speed;
    }
    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        map_state.rotation_x = (map_state.rotation_x + rotation_speed).clamp(-1.2, 1.2);
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        map_state.rotation_x = (map_state.rotation_x - rotation_speed).clamp(-1.2, 1.2);
    }

    // Zoom with Q/E or +/-
    if keys.pressed(KeyCode::KeyQ) || keys.pressed(KeyCode::Minus) {
        map_state.zoom = (map_state.zoom * (1.0 + time.delta_secs())).min(3.0);
    }
    if keys.pressed(KeyCode::KeyE) || keys.pressed(KeyCode::Equal) {
        map_state.zoom = (map_state.zoom * (1.0 - time.delta_secs())).max(0.3);
    }

    // Reset view with Home or R
    if keys.just_pressed(KeyCode::Home) || keys.just_pressed(KeyCode::KeyR) {
        map_state.rotation_y = 0.0;
        map_state.rotation_x = 0.3;
        map_state.zoom = 1.0;
    }

    // Update camera position to orbit around the galaxy center
    let distance = 20.0 * map_state.zoom;

    // Spherical coordinates for camera position
    let x = distance * map_state.rotation_x.cos() * map_state.rotation_y.sin();
    let y = distance * map_state.rotation_x.sin();
    let z = distance * map_state.rotation_x.cos() * map_state.rotation_y.cos();

    for mut transform in &mut camera_query {
        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

/// Handle clicking on stars to select/enter systems.
pub fn star_click_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), (With<GalaxyMapRoot>, With<Camera3d>)>,
    star_query: Query<(&StarMarker, &GlobalTransform)>,
    mut selection_query: Query<&mut Transform, (With<SelectionIndicator>, Without<StarMarker>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    map_state: Res<GalaxyMapState>,
    mut map_state_mut: ResMut<GalaxyMapState>,
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
            map_state_mut.selected_system = Some(idx);

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

/// Handle turn controls.
pub fn turn_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut map_state: ResMut<GalaxyMapState>,
    mut modal_state: ResMut<InfoModalState>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        map_state.turn_number += 1;
        info!("Turn {}", map_state.turn_number);

        // Demo: Show a notification every 5 turns
        if map_state.turn_number % 5 == 0 {
            *modal_state = InfoModalState::planet_notification(
                ModalIcon::Factory,
                format!(
                    "Factory construction complete on Terra Prime (Turn {})",
                    map_state.turn_number
                ),
                0,
                0,
            );
        }
    }

    // Press 'N' to show a test notification
    if keyboard.just_pressed(KeyCode::KeyN) {
        *modal_state = InfoModalState::planet_notification(
            ModalIcon::Factory,
            "Factory construction complete on Terra Prime",
            0,
            0,
        );
    }

    // Press 'M' to show a research notification
    if keyboard.just_pressed(KeyCode::KeyM) {
        *modal_state = InfoModalState::custom(
            ModalIcon::Research,
            "Research Complete: Advanced Propulsion",
            Some("Your scientists have discovered improved engine technology.".to_string()),
            vec![
                ModalButton {
                    label: "View Research".to_string(),
                    action: ModalAction::OpenResearch,
                },
                ModalButton {
                    label: "OK".to_string(),
                    action: ModalAction::Dismiss,
                },
            ],
        );
    }
}
