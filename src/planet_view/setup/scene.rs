use crate::planet_data::{BuildingType, PlanetSurface, TileColor};
use crate::planet_view::types::{BuildingEntity, PlanetView3D, TileEntity};
use crate::data_types::GameData;
use bevy::camera::ScalingMode;
use bevy::core_pipeline::core_3d::graph::Core3d;
use bevy::render::camera::CameraRenderGraph;
use bevy::prelude::*;
use std::collections::HashMap;

/// Set up the 3D scene with camera, lights, and planet grid.
pub fn setup_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    surface: &PlanetSurface,
    ambient_light: &mut ResMut<AmbientLight>,
    game_data: &GameData,
) {
    // Configure ambient light via resource (not as entity component due to Bevy 0.17 bug)
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 500.0;

    // Isometric Camera
    commands.spawn((
        Camera3d::default(),
        CameraRenderGraph::new(Core3d),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlanetView3D,
    ));

    // Lights - Note: AmbientLight should be configured as a resource, not spawned as entity
    // (In Bevy 0.17, AmbientLight requires Camera which causes spurious warnings)
    commands.spawn((
        DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlanetView3D,
    ));

    // Grid
    let tile_size = 1.0;
    let gap = 0.1;
    let offset_x = -(surface.row_width as f32 * (tile_size + gap)) / 2.0;
    let offset_z = -(surface.height() as f32 * (tile_size + gap)) / 2.0;

    let mesh_handle = meshes.add(Cuboid::new(tile_size, 0.2, tile_size));

    let white_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    let black_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        ..default()
    });

    // Building materials from GameData
    let mut building_materials = HashMap::new();
    for building_def in &game_data.surface_buildings {
        let (r, g, b) = building_def.color;
        let mat = materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            ..default()
        });
        building_materials.insert(building_def.id.clone(), mat);
    }

    let building_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.6));

    for (i, tile) in surface.tiles.iter().enumerate() {
        let x = i % surface.row_width;
        let y = i / surface.row_width;

        let pos_x = offset_x + x as f32 * (tile_size + gap);
        let pos_z = offset_z + y as f32 * (tile_size + gap);

        let mat = match tile.color {
            TileColor::White => white_mat.clone(),
            TileColor::Black => black_mat.clone(),
        };

        // Spawn Tile
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(mat),
            Transform::from_xyz(pos_x, 0.0, pos_z),
            PlanetView3D,
            TileEntity { x, y },
        ));

        // Spawn Building if present
        if let Some(building) = tile.building {
            let building_id = match building {
                BuildingType::Base => "building_base",
                BuildingType::Farm => "building_farm_1",
                BuildingType::Habitat => "building_habitat_1",
                BuildingType::Factory => "building_factory_1",
                BuildingType::Laboratory => "building_laboratory_1",
                BuildingType::Passage => "building_passage",
                BuildingType::Terraformer => "building_terraformer",
            };

            if let Some(b_mat) = building_materials.get(building_id) {
                commands.spawn((
                    Mesh3d(building_mesh.clone()),
                    MeshMaterial3d(b_mat.clone()),
                    Transform::from_xyz(pos_x, 0.4, pos_z),
                    PlanetView3D,
                    BuildingEntity,
                ));
            } else {
                warn!("Missing material for building ID: {}", building_id);
            }
        }
    }
}
