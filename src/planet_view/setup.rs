//! Planet view setup and initialization.
//!
//! Contains the main setup function that creates the 3D scene and UI overlay.

use bevy::camera::ScalingMode;
use bevy::prelude::*;

use crate::{GalaxyPreview, planet::PlanetSurface, star_system::StarSystemState};

use super::rendering::{create_planet_grid_mesh, create_planet_material, spawn_surface_buildings};
use super::types::{PlanetGrid, PlanetView3D, PlanetViewRoot, PlanetViewState, colors};
use super::ui::{spawn_left_panel, spawn_right_panel, spawn_top_bar};

/// Set up the planet view screen.
pub fn setup_planet_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    star_system_state: Res<StarSystemState>,
    galaxy_preview: Res<GalaxyPreview>,
    mut _planet_state: ResMut<PlanetViewState>,
) {
    // Get current planet info from star system state
    let system_index = star_system_state.system_index;
    let planet_index = star_system_state.selected_planet.unwrap_or(0);

    // Get planet data if available
    let (planet_name, surface_type, planet_size, surface_slots, orbital_slots, tiles, row_width) =
        galaxy_preview
            .galaxy
            .systems
            .get(system_index)
            .and_then(|s| s.planets.get(planet_index))
            .map(|p| {
                let _surface = PlanetSurface::from(p);
                (
                    format!("Planet {}", planet_index + 1),
                    p.surface_type_id.clone(),
                    p.size_id.clone(),
                    p.surface_slots,
                    p.orbital_slots,
                    p.tiles.clone(),
                    p.row_width,
                )
            })
            .unwrap_or_else(|| {
                (
                    "Unknown Planet".to_string(),
                    "unknown".to_string(),
                    "unknown".to_string(),
                    0,
                    0,
                    Vec::new(),
                    1,
                )
            });

    let num_planets = galaxy_preview
        .galaxy
        .systems
        .get(system_index)
        .map(|s| s.planets.len())
        .unwrap_or(0);

    // =========================================================================
    // 3D Scene Setup
    // =========================================================================

    setup_3d_scene(
        &mut commands,
        &mut meshes,
        &mut materials,
        &tiles,
        row_width,
        &surface_type,
    );

    // =========================================================================
    // 2D UI Overlay
    // =========================================================================

    setup_ui_overlay(
        &mut commands,
        num_planets,
        planet_index,
        system_index,
        &galaxy_preview,
        &planet_name,
        &surface_type,
        &planet_size,
        surface_slots as usize,
        orbital_slots as usize,
    );
}

/// Set up the 3D scene with camera, lights, and planet mesh.
fn setup_3d_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tiles: &[crate::planet::TileColor],
    row_width: usize,
    surface_type: &str,
) {
    // Isometric Camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 15.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Camera {
            order: 0, // Render first (background)
            clear_color: ClearColorConfig::Custom(colors::BACKGROUND),
            ..default()
        },
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlanetView3D,
    ));

    // Ambient light for base illumination
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 300.0,
            ..default()
        },
        PlanetView3D,
    ));

    // Directional light (sun-like)
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlanetView3D,
    ));

    // Create planet grid with colored tiles
    let grid_mesh = create_planet_grid_mesh(meshes, tiles, row_width);
    let planet_material = create_planet_material(materials, surface_type);

    // Planet grid entity
    commands.spawn((
        Mesh3d(grid_mesh),
        MeshMaterial3d(planet_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PlanetGrid,
        PlanetView3D,
    ));

    // Spawn building cubes on the surface (for tiles that have special features)
    spawn_surface_buildings(commands, meshes, materials, tiles, row_width);
}

/// Set up the 2D UI overlay.
#[allow(clippy::too_many_arguments)]
fn setup_ui_overlay(
    commands: &mut Commands,
    num_planets: usize,
    planet_index: usize,
    system_index: usize,
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
                system_index,
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
