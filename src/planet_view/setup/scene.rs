use bevy::prelude::*;
use bevy::camera::ScalingMode;
use crate::planet_view::rendering::{create_planet_grid_mesh, create_planet_material, spawn_surface_buildings};
use crate::planet_view::types::{PlanetGrid, PlanetView3D, colors};

/// Set up the 3D scene with camera, lights, and planet mesh.
pub fn setup_3d_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tiles: &[crate::planet_data::TileColor],
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
