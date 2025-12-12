//! Setup system for the galaxy map.
//!
//! Contains the main setup_galaxy_view function that spawns all initial
//! entities: camera, lights, stars, lanes, UI panels, etc.

pub mod lanes;
pub mod selection;
pub mod stars;
pub mod ui;

use bevy::{camera::ScalingMode, prelude::*};

use crate::galaxy_view::generation::generate_star_positions;
use crate::galaxy_view::types::{GalaxyViewRoot, GalaxyView3D};

use self::lanes::spawn_star_lanes;
use self::selection::spawn_selection_indicator;
use self::stars::{spawn_background_stars, spawn_stars};
use self::ui::{spawn_instructions, spawn_player_icon, spawn_ui_panel};

/// Setup the galaxy map screen.
pub fn setup_galaxy_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    galaxy_preview: Res<crate::GalaxyPreview>,
) {
    // 3D Camera with orthographic projection for the galaxy view
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 25.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.02, 0.05)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        GalaxyViewRoot,
        GalaxyView3D,
    ));

    // Ambient light
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 300.0,
            affects_lightmapped_meshes: false,
        },
        GalaxyViewRoot,
    ));

    // Point light for depth effect
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.95, 0.9),
            intensity: 50000.0,
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 15.0),
        GalaxyViewRoot,
    ));

    // Generate star positions from actual galaxy data
    let star_positions = generate_star_positions(&galaxy_preview.galaxy, 1337);

    // Spawn stars
    spawn_stars(
        &mut commands,
        &mut meshes,
        &mut materials,
        &star_positions,
    );

    // Add background stars (tiny distant dots)
    spawn_background_stars(&mut commands, &mut meshes, &mut materials);

    // Generate and spawn star lanes (connections between nearby stars)
    spawn_star_lanes(&mut commands, &mut meshes, &mut materials, &star_positions);

    // Selection indicator (hexagonal ring around selected star)
    spawn_selection_indicator(&mut commands, &mut meshes, &mut materials);

    // UI overlay - right side panel
    spawn_ui_panel(&mut commands);

    // Player icon in top-left (species indicator)
    spawn_player_icon(&mut commands);

    // Instructions
    spawn_instructions(&mut commands);
}
