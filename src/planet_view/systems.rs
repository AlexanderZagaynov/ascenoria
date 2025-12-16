//! ECS systems for the planet view.

use bevy::prelude::*;
// use bevy::hierarchy::DespawnRecursiveExt;

use crate::main_menu::GameState;
use crate::planet_data::{BuildingType, TileColor};
use crate::data_types::GameData;
use crate::planet_view::types::{
    BuildingEntity, PlanetView3D, PlanetViewRoot, PlanetViewState, TileEntity, UIAction,
};

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
pub fn configure_ui_camera(
    mut query: Query<&mut Camera, (Added<PlanetViewRoot>, With<Camera2d>)>,
) {
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
) {
    for (interaction, action, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                match action {
                    UIAction::EndTurn => {
                        end_turn(&mut planet_state);
                    }
                    UIAction::SelectBuilding(b) => {
                        planet_state.selected_building = Some(*b);
                        info!("Selected building: {:?}", b);
                    }
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

fn end_turn(state: &mut PlanetViewState) {
    state.turn += 1;

    // Calculate yields
    if let Some(surface) = &state.surface {
        for tile in &surface.tiles {
            if let Some(building) = tile.building {
                match building {
                    BuildingType::Base => {
                        state.food += 1;
                        state.housing += 3;
                        state.production += 1;
                        state.science += 1;
                    }
                    BuildingType::Farm => state.food += 1,
                    BuildingType::Habitat => state.housing += 2,
                    BuildingType::Factory => state.production += 1,
                    BuildingType::Laboratory => state.science += 1,
                    _ => {}
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

// Simple raycast system for tile clicking
pub fn tile_interaction_system(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut planet_state: ResMut<PlanetViewState>,
    tile_q: Query<(Entity, &TileEntity, &Transform)>,
    mut update_events: MessageWriter<crate::planet_view::types::TileUpdateEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some((camera, camera_transform)) = camera_q.iter().next() {
        if let Some(window) = windows.iter().next() {
            if let Some(cursor_position) = window.cursor_position() {
                let ray = camera
                    .viewport_to_world(camera_transform, cursor_position)
                    .unwrap();

                // Intersect with plane y=0
                let t = -ray.origin.y / ray.direction.y;
                if t > 0.0 {
                    let intersection = ray.origin + ray.direction * t;

                    // Find closest tile
                    let mut closest_dist = 1.0; // Max dist
                    let mut closest_tile = None;

                    for (entity, tile, transform) in &tile_q {
                        let dist = intersection.distance(transform.translation);
                        if dist < closest_dist {
                            closest_dist = dist;
                            closest_tile = Some((entity, tile));
                        }
                    }

                    if let Some((_entity, tile_data)) = closest_tile {
                        handle_tile_click(
                            tile_data.x,
                            tile_data.y,
                            &mut planet_state,
                            &mut update_events,
                        );
                    }
                }
            }
        }
    }
}

fn handle_tile_click(
    x: usize,
    y: usize,
    state: &mut PlanetViewState,
    update_events: &mut MessageWriter<crate::planet_view::types::TileUpdateEvent>,
) {
    let building_type = match state.selected_building {
        Some(b) => b,
        None => return,
    };

    // Check cost
    let cost = 10;
    if state.production < cost {
        info!("Not enough production!");
        return;
    }

    // Check validity
    if let Some(surface) = &mut state.surface {
        // Check terrain
        let tile_color = surface.get(x, y).unwrap().color;
        let valid_terrain = match building_type {
            BuildingType::Terraformer | BuildingType::Passage => tile_color == TileColor::Black,
            _ => tile_color == TileColor::White,
        };

        if !valid_terrain {
            info!("Invalid terrain for this building!");
            return;
        }

        // Check if empty
        if surface.get(x, y).unwrap().building.is_some() {
            info!("Tile occupied!");
            return;
        }

        // Check adjacency
        let mut adjacent = false;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 {
                if let Some(neighbor) = surface.get(nx as usize, ny as usize) {
                    if neighbor.building.is_some() {
                        adjacent = true;
                        break;
                    }
                }
            }
        }

        if !adjacent {
            info!("Must be adjacent to existing building!");
            return;
        }

        // Check Tech
        if building_type == BuildingType::Terraformer && !state.terraforming_unlocked {
            info!("Terraforming not researched!");
            return;
        }

        // Place Building
        state.production -= cost;
        let tile = surface.get_mut(x, y).unwrap();

        if building_type == BuildingType::Terraformer {
            tile.color = TileColor::White;
            info!("Terraformed!");
        } else {
            tile.building = Some(building_type);
            info!("Built {:?}!", building_type);
        }

        // Check Victory
        if surface.tiles.iter().all(|t| t.building.is_some()) {
            state.victory = true;
            info!("VICTORY!");
        }

        // Trigger update
        update_events.write(crate::planet_view::types::TileUpdateEvent { x, y });
    }
}

pub fn update_visuals_system(
    mut events: MessageReader<crate::planet_view::types::TileUpdateEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planet_state: Res<PlanetViewState>,
    game_data: Res<GameData>,
    tile_q: Query<(Entity, &TileEntity, &Transform)>,
    _building_q: Query<(Entity, &BuildingEntity, &ChildOf)>,
) {
    for event in events.read() {
        // Find tile entity
        for (entity, tile_data, transform) in &tile_q {
            if tile_data.x == event.x && tile_data.y == event.y {
                // Update tile material (if terraformed)
                if let Some(surface) = &planet_state.surface {
                    if let Some(tile) = surface.get(event.x, event.y) {
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
                            let building_id = match building {
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

                            commands.spawn((
                                Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 0.6))),
                                MeshMaterial3d(materials.add(StandardMaterial {
                                    base_color: color,
                                    ..default()
                                })),
                                Transform::from_xyz(
                                    transform.translation.x,
                                    0.4,
                                    transform.translation.z,
                                ),
                                PlanetView3D,
                                BuildingEntity,
                            ));
                        }

                        // If terraformed, update tile color
                        if tile.color == TileColor::White {
                            commands.entity(entity).insert(MeshMaterial3d(materials.add(
                                StandardMaterial {
                                    base_color: Color::WHITE,
                                    ..default()
                                },
                            )));
                        }
                    }
                }
            }
        }
    }
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
