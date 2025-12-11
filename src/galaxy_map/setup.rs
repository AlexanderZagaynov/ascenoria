//! Setup system for the galaxy map.
//!
//! Contains the main setup_galaxy_map function that spawns all initial
//! entities: camera, lights, stars, lanes, UI panels, etc.

use bevy::{camera::ScalingMode, prelude::*};
use rand::{Rng, SeedableRng, rngs::StdRng};

use super::colors;
use super::generation::{create_lane_mesh, generate_star_lanes, generate_star_positions};
use super::types::{
    GalaxyMapRoot, GalaxyView3D, PanelButton, SelectionIndicator, StarLane, StarMarker,
};
use super::ui::{
    spawn_bottom_controls, spawn_panel_section, spawn_speed_controls, spawn_turn_indicators,
};

/// Setup the galaxy map screen.
pub fn setup_galaxy_map(
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
        GalaxyMapRoot,
        GalaxyView3D,
    ));

    // Ambient light
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 300.0,
            affects_lightmapped_meshes: false,
        },
        GalaxyMapRoot,
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
        GalaxyMapRoot,
    ));

    // Generate star positions from actual galaxy data
    let star_positions = generate_star_positions(&galaxy_preview.galaxy, 1337);

    // Create star mesh (small sphere)
    let star_mesh = meshes.add(Sphere::new(0.2));
    let glow_mesh = meshes.add(Sphere::new(0.35));

    // Spawn stars
    for (i, (pos, star_type, _name)) in star_positions.iter().enumerate() {
        // Main star material with emission
        let star_material = materials.add(StandardMaterial {
            base_color: star_type.color(),
            emissive: star_type.color().to_linear() * 5.0,
            ..default()
        });

        // Glow material
        let glow_material = materials.add(StandardMaterial {
            base_color: star_type.color().with_alpha(0.3),
            emissive: star_type.color().to_linear() * 2.0,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        // Main star
        commands.spawn((
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(star_material),
            Transform::from_translation(*pos),
            StarMarker { system_index: i },
            GalaxyMapRoot,
            GalaxyView3D,
        ));

        // Glow effect
        commands.spawn((
            Mesh3d(glow_mesh.clone()),
            MeshMaterial3d(glow_material),
            Transform::from_translation(*pos),
            GalaxyMapRoot,
            GalaxyView3D,
        ));
    }

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

fn spawn_background_stars(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let bg_star_mesh = meshes.add(Sphere::new(0.03));
    let bg_star_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.7, 0.7, 0.8, 0.5),
        emissive: bevy::color::LinearRgba::rgb(0.2, 0.2, 0.3),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let mut bg_rng = StdRng::seed_from_u64(42);
    for _ in 0..200 {
        let x = Rng::gen_range(&mut bg_rng, -15.0..15.0);
        let y = Rng::gen_range(&mut bg_rng, -8.0..8.0);
        let z = Rng::gen_range(&mut bg_rng, -15.0..15.0);

        commands.spawn((
            Mesh3d(bg_star_mesh.clone()),
            MeshMaterial3d(bg_star_material.clone()),
            Transform::from_xyz(x, y, z),
            GalaxyMapRoot,
            GalaxyView3D,
        ));
    }
}

fn spawn_star_lanes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    star_positions: &[(Vec3, super::types::StarType, String)],
) {
    let lanes = generate_star_lanes(star_positions, 5.0);
    let lane_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.5, 0.9, 0.6),
        emissive: bevy::color::LinearRgba::rgb(0.1, 0.3, 0.6),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for (i, j) in &lanes {
        let (lane_mesh, lane_transform) =
            create_lane_mesh(star_positions[*i].0, star_positions[*j].0);
        commands.spawn((
            Mesh3d(meshes.add(lane_mesh)),
            MeshMaterial3d(lane_material.clone()),
            lane_transform,
            StarLane,
            GalaxyMapRoot,
            GalaxyView3D,
        ));
    }
}

fn spawn_selection_indicator(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let selection_mesh = meshes.add(Torus::new(0.3, 0.35));
    let selection_material = materials.add(StandardMaterial {
        base_color: colors::RING_GREEN.with_alpha(0.8),
        emissive: bevy::color::LinearRgba::rgb(0.2, 0.8, 0.3),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(selection_mesh),
        MeshMaterial3d(selection_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)) // Hidden initially
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        SelectionIndicator,
        GalaxyMapRoot,
        GalaxyView3D,
    ));
}

fn spawn_ui_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            GalaxyMapRoot,
        ))
        .with_children(|parent| {
            // Right panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                ))
                .with_children(|panel| {
                    // Turn indicators at top
                    spawn_turn_indicators(panel);

                    // Speed controls
                    spawn_speed_controls(panel);

                    // Main menu buttons
                    spawn_panel_section(panel, "Planets", PanelButton::Planets);
                    spawn_panel_section(panel, "Ships", PanelButton::Ships);
                    spawn_panel_section(panel, "Research", PanelButton::Research);
                    spawn_panel_section(panel, "Special Ability", PanelButton::SpecialAbility);
                    spawn_panel_section(panel, "Species", PanelButton::Species);

                    // Spacer
                    panel.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });

                    // Bottom control buttons
                    spawn_bottom_controls(panel);
                });
        });
}

fn spawn_player_icon(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::RING_GREEN),
            GalaxyMapRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::RING_GREEN),
            ));
        });
}

fn spawn_instructions(commands: &mut Commands) {
    commands.spawn((
        Text::new("Rotate: LMB/RMB drag / Arrow keys / WASD • Zoom: Q/E • Reset: R • Click star to select, twice to enter"),
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
        GalaxyMapRoot,
    ));
}
