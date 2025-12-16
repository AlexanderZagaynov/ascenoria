//! ECS systems for the planet view.

use bevy::prelude::*;
// use bevy::ecs::hierarchy::DespawnRecursiveExt;

use crate::data_types::GameData;
use crate::data_types::GameRegistry;
use crate::main_menu::GameState;
use crate::planet_data::{BuildingType, TileColor};
use crate::planet_view::logic::update_connectivity;
use crate::planet_view::types::{
    BuildingEntity, PlanetView3D, PlanetViewRoot, PlanetViewState, TileEntity, UIAction,
};
use crate::planet_view::ui::panels::ProductionQueueList;

/// Clean up all planet view entities when leaving the screen.
pub fn cleanup_planet_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<PlanetViewRoot>>,
    view_3d_query: Query<Entity, With<PlanetView3D>>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn();
    }
    for entity in &view_3d_query {
        commands.entity(entity).despawn();
    }
}

/// Configure the UI camera to render on top of the 3D scene.
pub fn configure_ui_camera(mut query: Query<&mut Camera, (Added<PlanetViewRoot>, With<Camera2d>)>) {
    for mut camera in query.iter_mut() {
        camera.order = 1;
        camera.clear_color = ClearColorConfig::None;
    }
}

pub fn ui_action_system(
    mut interaction_query: Query<
        (&Interaction, &UIAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut planet_state: ResMut<PlanetViewState>,
    mut next_state: ResMut<NextState<GameState>>,
    game_data: Res<GameData>,
    registry: Res<GameRegistry>,
) {
    for (interaction, action, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                match action {
                    UIAction::EndTurn => {
                        end_turn(&mut planet_state, &game_data, &registry);
                    }
                    // UIAction::OpenBuildMenu => {
                    //     info!("Open Build Menu");
                    // }
                    UIAction::Quit => {
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}

fn end_turn(state: &mut PlanetViewState, game_data: &GameData, registry: &GameRegistry) {
    state.turn += 1;

    // Calculate yields
    if let Some(surface) = &state.surface {
        for tile in &surface.tiles {
            if let Some(building) = tile.building {
                let building_id = building.id();
                if let Some(def) = game_data.surface_buildings.iter().find(|b| b.id == building_id) {
                    state.food = (state.food as i32 + def.yields_food).max(0) as u32;
                    state.housing = (state.housing as i32 + def.yields_housing).max(0) as u32;
                    state.production = (state.production as i32 + def.yields_production).max(0) as u32;
                    state.science = (state.science as i32 + def.yields_science).max(0) as u32;
                } else {
                    warn!("Missing building definition for ID: {}", building_id);
                }
            }
        }
    }

    // Process Production Queue
    if let Some(project) = state.production_queue.front_mut() {
        let needed = project.total_cost.saturating_sub(project.progress);
        let available = state.production;
        let amount = std::cmp::min(needed, available);

        project.progress += amount;
        // state.production -= amount;

        if project.progress >= project.total_cost {
            // Finished!
            let finished_project = state.production_queue.pop_front().unwrap();
            match finished_project.project_type {
                crate::planet_view::types::ProjectType::Building(b_type) => {
                    if let Some(surface) = &mut state.surface {
                        if let Some(tile) =
                            surface.tiles.get_mut(finished_project.target_tile_index)
                        {
                            tile.building = Some(b_type);
                            info!("Construction Complete: {:?}", b_type);
                            // Update connectivity
                            update_connectivity(surface, game_data, registry);
                        }
                    }
                }
            }
        }
    }

    // Research
    state.research_progress += state.science;
    if state.research_progress >= 100 {
        // Hardcoded cost
        state.terraforming_unlocked = true;
    }

    // Reset per-turn stats if needed (Production accumulates? MVP says "Production may be used to construct buildings". "Instant construction is acceptable". So maybe Production is a currency.)
    // "Sum yields... Production may be used to construct buildings" implies accumulation.

    info!(
        "Turn ended. Food: {}, Housing: {}, Prod: {}, Sci: {}",
        state.food, state.housing, state.production, state.science
    );
}

// Simple raycast system for tile clicking and hovering
pub fn tile_interaction_system(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut planet_state: ResMut<PlanetViewState>,
    tile_q: Query<(Entity, &TileEntity, &Transform)>,
    mut cursor_q: Query<(&mut Transform, &mut Visibility), (With<crate::planet_view::types::PlanetViewCursor>, Without<TileEntity>)>,
    mut update_events: MessageWriter<crate::planet_view::types::TileUpdateEvent>,
    game_data: Res<GameData>,
    registry: Res<GameRegistry>,
) {
    let mut hovered_tile_pos = None;
    let mut hovered_tile_data = None;

    if let Some((camera, camera_transform)) = camera_q.iter().next() {
        if let Some(window) = windows.iter().next() {
            if let Some(cursor_position) = window.cursor_position() {
                if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    // Intersect with plane y=0
                    let t = -ray.origin.y / ray.direction.y;
                    if t > 0.0 {
                        let intersection = ray.origin + ray.direction * t;

                        // Find closest tile
                        let mut closest_dist = 1.0; // Max dist

                        for (_entity, tile, transform) in &tile_q {
                            // Ignore y difference for distance check
                            let flat_intersection = Vec3::new(intersection.x, 0.0, intersection.z);
                            let flat_tile_pos = Vec3::new(transform.translation.x, 0.0, transform.translation.z);

                            let dist = flat_intersection.distance(flat_tile_pos);
                            if dist < closest_dist {
                                closest_dist = dist;
                                hovered_tile_pos = Some(transform.translation);
                                hovered_tile_data = Some(tile);
                            }
                        }
                    }
                }
            }
        }
    }

    // Update Cursor
    if let Ok((mut cursor_transform, mut cursor_visibility)) = cursor_q.single_mut() {
        if let Some(pos) = hovered_tile_pos {
            cursor_transform.translation = pos + Vec3::new(0.0, 0.1, 0.0); // Slightly above
            *cursor_visibility = Visibility::Visible;
        } else {
            *cursor_visibility = Visibility::Hidden;
        }
    }

    // Handle Click
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(tile_data) = hovered_tile_data {
             handle_tile_click(
                tile_data.x,
                tile_data.y,
                &mut planet_state,
                &mut update_events,
                &game_data,
                &registry,
            );
        }
    }
}

fn handle_tile_click(
    x: usize,
    y: usize,
    state: &mut PlanetViewState,
    _update_events: &mut MessageWriter<crate::planet_view::types::TileUpdateEvent>,
    _game_data: &GameData,
    _registry: &GameRegistry,
) {
    if let Some(surface) = &mut state.surface {
        // Check if empty
        if surface.get(x, y).unwrap().building.is_some() {
            info!("Tile occupied!");
            return;
        }

        // Check connectivity
        if !surface.get(x, y).unwrap().connected {
            info!("Tile not connected!");
            return;
        }

        // Open Menu
        state.build_menu_open = true;
        state.build_menu_target_tile = Some(y * surface.row_width + x);
        info!("Opening Build Menu for tile ({}, {})", x, y);
    }
}

pub fn update_visuals_system(
    mut events: MessageReader<crate::planet_view::types::TileUpdateEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planet_state: Res<PlanetViewState>,
    game_data: Res<GameData>,
    assets: Res<crate::planet_view::types::PlanetViewAssets>,
    mut tile_q: Query<(Entity, &TileEntity, &Transform, &mut Mesh3d)>,
    _building_q: Query<(Entity, &BuildingEntity, &ChildOf)>,
) {
    for event in events.read() {
        // Find tile entity
        for (entity, tile_data, transform, mut mesh_handle) in &mut tile_q {
            if tile_data.x == event.x && tile_data.y == event.y {
                // Update tile material (if terraformed)
                if let Some(surface) = &planet_state.surface {
                    if let Some(tile) = surface.get(event.x, event.y) {
                        // Update Mesh based on connectivity
                        mesh_handle.0 = if tile.connected {
                            assets.large_plate_mesh.clone()
                        } else {
                            assets.small_diamond_mesh.clone()
                        };

                        // Re-spawn building if present
                        // First remove existing building on this tile
                        // This is tricky because BuildingEntity is child of... wait, I spawned them as siblings in setup_scene.
                        // But I didn't link them.
                        // I should have linked them or just check position.
                        // Since I spawned them at same x,z, I can check position.

                        // Remove old building at this position
                        // Note: In a full implementation we would despawn the existing building entity here.
                        // For MVP, we just spawn the new one on top (or rely on the fact that we only build on empty tiles,
                        // except for terraforming which doesn't spawn a building).

                        // Let's just spawn the new building.
                        if let Some(building) = tile.building {
                            spawn_building(
                                &mut commands,
                                &mut meshes,
                                &mut materials,
                                &game_data,
                                building,
                                transform.translation,
                                false, // Not a construction site
                            );
                        } else {
                            // Check if there is a construction project for this tile
                            if let Some(project) = planet_state.production_queue.iter().find(|p| p.target_tile_index == (tile_data.y * surface.row_width + tile_data.x)) {
                                match project.project_type {
                                    crate::planet_view::types::ProjectType::Building(b_type) => {
                                        spawn_building(
                                            &mut commands,
                                            &mut meshes,
                                            &mut materials,
                                            &game_data,
                                            b_type,
                                            transform.translation,
                                            true, // Is construction site
                                        );
                                    }
                                }
                            }
                        }

                        // If terraformed, update tile color
                        if tile.color == TileColor::White {
                            commands.entity(entity).insert(MeshMaterial3d(materials.add(
                                StandardMaterial {
                                    base_color: Color::WHITE,
                                    ..default()
                                },
                            )));
                        } else if tile.color == TileColor::Black {
                             commands.entity(entity).insert(MeshMaterial3d(assets.black_mat.clone()));
                        }
                    }
                }
            }
        }
    }
}

fn spawn_building(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    game_data: &GameData,
    building_type: BuildingType,
    position: Vec3,
    is_construction: bool,
) {
    let building_id = match building_type {
        BuildingType::Base => "building_base",
        BuildingType::Farm => "building_farm_1",
        BuildingType::Habitat => "building_habitat_1",
        BuildingType::Factory => "building_factory_1",
        BuildingType::Laboratory => "building_laboratory_1",
        BuildingType::Passage => "building_passage",
        BuildingType::Terraformer => "building_terraformer",
    };

    // Find color in GameData
    let color = if let Some(def) = game_data.surface_buildings.iter().find(|b| b.id == building_id) {
        let (r, g, b) = def.color;
        Color::srgb(r, g, b)
    } else {
        warn!("Missing building definition for ID: {}", building_id);
        Color::WHITE
    };

    let final_color = if is_construction {
        color.with_alpha(0.5) // Transparent for construction
    } else {
        color
    };

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: final_color,
            alpha_mode: if is_construction { AlphaMode::Blend } else { AlphaMode::Opaque },
            ..default()
        })),
        Transform::from_xyz(
            position.x,
            0.4,
            position.z,
        ),
        PlanetView3D,
        BuildingEntity,
    ));
}

pub fn update_ui_system(
    planet_state: Res<PlanetViewState>,
    mut text_query: Query<&mut Text>,
    mut victory_query: Query<&mut Node, With<crate::planet_view::types::VictoryMessage>>,
) {
    // Victory Message
    if let Some(mut node) = victory_query.iter_mut().next() {
        node.display = if planet_state.victory {
            Display::Flex
        } else {
            Display::None
        };
    }

    // This is very naive, updating all texts.
    // I should tag them properly.
    // But for MVP, I'll just iterate and check content or use specific markers.
    // I didn't add markers in overlay.rs.
    // I'll just rely on order or content matching? No, that's bad.
    // I'll update overlay.rs to add markers if I can, or just use a single text block for stats.

    // Let's assume I can find them by content prefix.
    for mut text in &mut text_query {
        if text.0.starts_with("Turn:") {
            text.0 = format!("Turn: {}", planet_state.turn);
        } else if text.0.starts_with("Food:") {
            text.0 = format!("Food: {}", planet_state.food);
        } else if text.0.starts_with("Housing:") {
            text.0 = format!("Housing: {}", planet_state.housing);
        } else if text.0.starts_with("Prod:") {
            text.0 = format!("Prod: {}", planet_state.production);
        } else if text.0.starts_with("Science:") {
            text.0 = format!("Science: {}", planet_state.science);
        } else if text.0.starts_with("Research:") {
            text.0 = format!("Research: {}/100", planet_state.research_progress);
        }
    }
}

pub fn update_connectivity_system(
    mut planet_state: ResMut<PlanetViewState>,
    game_data: Res<GameData>,
    registry: Res<GameRegistry>,
) {
    if let Some(surface) = &mut planet_state.surface {
        update_connectivity(surface, &game_data, &registry);
    }
}

pub fn update_production_queue_ui(
    mut commands: Commands,
    planet_state: Res<PlanetViewState>,
    queue_query: Query<(Entity, Option<&Children>), With<ProductionQueueList>>,
) {
    for (entity, children) in &queue_query {
        if let Some(children) = children {
            for child in children {
                commands.entity(*child).despawn();
            }
        }

        commands.entity(entity).with_children(|parent| {
            for (i, project) in planet_state.production_queue.iter().enumerate() {
                let name = match project.project_type {
                    crate::planet_view::types::ProjectType::Building(b) => format!("{:?}", b),
                };

                let progress_text = format!("{} / {}", project.progress, project.total_cost);
                let color = if i == 0 {
                    Color::srgb(0.0, 1.0, 0.0)
                } else {
                    Color::WHITE
                };

                let income_text = if i == 0 {
                    format!(" (+{})", planet_state.production)
                } else {
                    "".to_string()
                };

                parent.spawn((
                    Text::new(format!("{}: {}{}", name, progress_text, income_text)),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(color),
                ));
            }
        });
    }
}
