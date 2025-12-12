use bevy::prelude::*;
use crate::star_system::types::{GridPlane, StarSystemRoot, colors};

/// Draw the isometric grid plane.
pub fn spawn_grid_plane(commands: &mut Commands) {
    let grid_size = 12;
    let cell_size = 40.0;
    let offset_x = -(grid_size as f32 * cell_size) / 2.0;
    let offset_y = -150.0; // Below center

    // Draw grid lines (simplified isometric - horizontal lines with slight perspective)
    for i in 0..=grid_size {
        let y_offset = i as f32 * (cell_size * 0.5);
        let x_start = offset_x - i as f32 * 10.0;
        let x_end = -offset_x + i as f32 * 10.0;

        // Horizontal-ish lines (going "into" the screen)
        let line_color = if i == grid_size / 2 {
            colors::GRID_HIGHLIGHT
        } else {
            colors::GRID_LINE
        };

        commands.spawn((
            Sprite {
                color: line_color.with_alpha(0.6),
                custom_size: Some(Vec2::new((x_end - x_start).abs(), 1.5)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                (x_start + x_end) / 2.0,
                offset_y + y_offset,
                0.1,
            )),
            GridPlane,
            StarSystemRoot,
        ));
    }

    // Vertical-ish lines (going "across")
    for i in 0..=grid_size {
        let x_pos = offset_x + i as f32 * cell_size;
        let y_start = offset_y;
        let y_end = offset_y + grid_size as f32 * (cell_size * 0.5);

        let line_color = if i == grid_size / 2 {
            colors::GRID_HIGHLIGHT
        } else {
            colors::GRID_LINE
        };

        // Slight slant for perspective
        let slant = (i as f32 - grid_size as f32 / 2.0) * 0.8;

        commands.spawn((
            Sprite {
                color: line_color.with_alpha(0.5),
                custom_size: Some(Vec2::new(1.5, (y_end - y_start).abs())),
                ..default()
            },
            Transform::from_translation(Vec3::new(x_pos + slant, (y_start + y_end) / 2.0, 0.1)),
            GridPlane,
            StarSystemRoot,
        ));
    }
}
