use bevy::prelude::*;
use crate::GalaxyPreview;
use crate::planet_view::types::PlanetViewRoot;
use crate::planet_view::ui::{spawn_left_panel, spawn_right_panel, spawn_top_bar};

/// Set up the 2D UI overlay.
#[allow(clippy::too_many_arguments)]
pub fn setup_ui_overlay(
    commands: &mut Commands,
    num_planets: usize,
    planet_index: usize,
    star_index: usize,
    galaxy_preview: &GalaxyPreview,
    planet_name: &str,
    surface_type: &str,
    planet_size: &str,
    surface_slots: usize,
    orbital_slots: usize,
) {
    // 2D Camera for UI overlay
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // Render on top
            clear_color: ClearColorConfig::None,
            ..default()
        },
        PlanetViewRoot,
    ));

    // Root container - full screen UI overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // Transparent background - 3D scene shows through
            BackgroundColor(Color::NONE),
            PlanetViewRoot,
        ))
        .with_children(|root| {
            // Top bar - planet thumbnails and info
            spawn_top_bar(
                root,
                num_planets,
                planet_index,
                star_index,
                galaxy_preview,
                planet_name,
                surface_type,
                planet_size,
            );

            // Main content area - sides only (center is transparent for 3D)
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            })
            .with_children(|main| {
                // Left panel - Planet info
                spawn_left_panel(
                    main,
                    planet_name,
                    surface_type,
                    planet_size,
                    surface_slots,
                    orbital_slots,
                );

                // Center area - transparent (3D shows through)
                main.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                });

                // Right panel - Orbital structures
                spawn_right_panel(main, orbital_slots);
            });
        });
}
