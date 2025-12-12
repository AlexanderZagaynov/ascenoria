use bevy::prelude::*;
use crate::star_system::generation::generate_planet_positions;
use crate::star_system::types::{
    PlanetMarker, PlanetStalk, StarSystemRoot, StarSystemState, colors, to_roman,
};
use crate::star_system::ui::{spawn_system_label, spawn_ui_panel};
use super::grid::spawn_grid_plane;
use super::background::spawn_background_stars;

/// Set up the star system view screen.
pub fn setup_star_system(
    mut commands: Commands,
    galaxy_preview: Res<crate::GalaxyPreview>,
    state: Res<StarSystemState>,
) {
    // Camera for the system view
    commands.spawn((Camera2d::default(), StarSystemRoot));

    // Background (space)
    commands.spawn((
        Sprite {
            color: colors::SPACE_BLACK,
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        StarSystemRoot,
    ));

    // Draw the grid plane
    spawn_grid_plane(&mut commands);

    // Generate and spawn planets with stalks
    let positions = generate_planet_positions(&galaxy_preview.galaxy, state.system_index, 42);

    // Get system info for the panel
    let system = galaxy_preview.galaxy.systems.get(state.system_index);
    let system_name = system.map(|s| s.name.as_str()).unwrap_or("Unknown System");

    for (i, pos) in positions.iter().enumerate() {
        let _planet = system.and_then(|s| s.planets.get(i));

        // Calculate isometric screen position
        // X stays as X, Y combines height and Z depth
        let screen_x = pos.grid_pos.x;
        let screen_y = pos.grid_pos.y + pos.height;

        // Draw the stalk (vertical line from grid to planet)
        let stalk_base_y = pos.grid_pos.y;
        let stalk_height = pos.height;

        // Stalk sprite (stretched vertical line)
        commands.spawn((
            Sprite {
                color: colors::STALK_COLOR,
                custom_size: Some(Vec2::new(3.0, stalk_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                screen_x,
                stalk_base_y + stalk_height / 2.0,
                0.5,
            )),
            PlanetStalk { planet_index: i },
            StarSystemRoot,
        ));

        // Small base marker on grid
        commands.spawn((
            Sprite {
                color: colors::STALK_COLOR.with_alpha(0.8),
                custom_size: Some(Vec2::splat(8.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(screen_x, stalk_base_y, 0.4)),
            StarSystemRoot,
        ));

        // Planet sphere (represented as circle)
        commands.spawn((
            Sprite {
                color: pos.visual.primary_color(),
                custom_size: Some(Vec2::splat(pos.size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(screen_x, screen_y, 1.0)),
            PlanetMarker { planet_index: i },
            StarSystemRoot,
        ));

        // Planet highlight/atmosphere glow
        commands.spawn((
            Sprite {
                color: pos.visual.primary_color().with_alpha(0.3),
                custom_size: Some(Vec2::splat(pos.size * 1.3)),
                ..default()
            },
            Transform::from_translation(Vec3::new(screen_x, screen_y, 0.9)),
            StarSystemRoot,
        ));

        // Planet label (name)
        let planet_name = format!("{} {}", system_name.replace("System-", ""), to_roman(i + 1));
        commands.spawn((
            Text::new(&planet_name),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::PLANET_LABEL),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(screen_x + 400.0 + pos.size / 2.0 + 10.0), // Offset from center
                top: Val::Px(300.0 - screen_y - 5.0),                    // Invert Y for UI
                ..default()
            },
            StarSystemRoot,
        ));
    }

    // Add central star/galaxy indicator
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.4).with_alpha(0.9),
            custom_size: Some(Vec2::splat(60.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-50.0, 80.0, 0.8)),
        StarSystemRoot,
    ));

    // Star glow
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.9, 0.5).with_alpha(0.4),
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-50.0, 80.0, 0.7)),
        StarSystemRoot,
    ));

    // Spawn background stars
    spawn_background_stars(&mut commands);

    // UI overlay - right side panel
    spawn_ui_panel(&mut commands, &galaxy_preview.galaxy, state.system_index);

    // System name in top-left
    spawn_system_label(&mut commands, system_name);

    // Instructions at bottom
    commands.spawn((
        Text::new("Click planets to select • Arrow keys to rotate • ESC for galaxy map"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::PANEL_TEXT_DIM),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        StarSystemRoot,
    ));
}
