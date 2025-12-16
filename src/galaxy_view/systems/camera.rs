use bevy::{prelude::*, window::PrimaryWindow};

use crate::galaxy_view::types::{GalaxyViewRoot, GalaxyViewState};

/// Handle galaxy rotation via mouse drag and camera orbit.
pub fn galaxy_rotation_system(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut map_state: ResMut<GalaxyViewState>,
    mut camera_query: Query<&mut Transform, (With<GalaxyViewRoot>, With<Camera3d>)>,
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
