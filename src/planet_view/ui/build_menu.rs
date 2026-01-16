//! Build menu modal for selecting buildings to construct.
//!
//! This module implements the popup menu that appears when a player
//! clicks on a valid (connected, empty) tile. It displays available
//! building types and adds selected buildings to the production queue.

use bevy::prelude::*;
use crate::data_types::GameData;
use crate::planet_data::BuildingType;
use crate::planet_view::types::{PlanetViewState, ProductionProject, ProjectType};

/// Marker component for the build menu root entity.
///
/// Used to find and despawn the menu when it should be closed.
#[derive(Component)]
pub struct BuildMenuRoot;

/// Component attached to building selection buttons.
///
/// Contains the building type that will be added to the queue when clicked.
#[derive(Component)]
pub struct BuildMenuAction(pub BuildingType);

/// Marker component for the cancel button.
#[derive(Component)]
pub struct BuildMenuCancel;

/// System to show/hide the build menu based on game state.
///
/// - Spawns the menu when `build_menu_open` becomes true
/// - Despawns the menu when `build_menu_open` becomes false
pub fn update_build_menu(
    mut commands: Commands,
    planet_state: Res<PlanetViewState>,
    menu_query: Query<Entity, With<BuildMenuRoot>>,
    game_data: Res<GameData>,
) {
    let is_open = planet_state.build_menu_open;
    let has_menu = !menu_query.is_empty();

    if is_open && !has_menu {
        // Menu should be open but doesn't exist - spawn it
        spawn_build_menu(&mut commands, &game_data);
    } else if !is_open && has_menu {
        // Menu should be closed but exists - despawn it
        for entity in &menu_query {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn the build menu UI hierarchy.
///
/// Creates a centered modal dialog with:
/// - Title text
/// - List of building type buttons
/// - Cancel button at the bottom
fn spawn_build_menu(commands: &mut Commands, _game_data: &GameData) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(30.0),
                top: Val::Percent(20.0),
                width: Val::Percent(40.0),
                height: Val::Percent(60.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.9)),
            BorderColor::all(Color::WHITE),
            BuildMenuRoot,
            GlobalZIndex(10),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Select Building"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // List of buildings
            let buildings = vec![
                (BuildingType::Farm, "Farm"),
                (BuildingType::Habitat, "Habitat"),
                (BuildingType::Factory, "Factory"),
                (BuildingType::Laboratory, "Laboratory"),
                (BuildingType::Passage, "Passage"),
                (BuildingType::Terraformer, "Terraformer"),
            ];

            for (b_type, name) in buildings {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        BuildMenuAction(b_type),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(name),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }

            // Cancel Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
                    BuildMenuCancel,
                ))
                .with_children(|btn| {
                    btn.spawn((Text::new("Cancel"), TextColor(Color::WHITE)));
                });
        });
}

/// System to handle button clicks in the build menu.
///
/// # Building Selection
/// When a building button is clicked:
/// 1. Creates a `ProductionProject` with the selected building type
/// 2. Sets the target tile from `build_menu_target_tile`
/// 3. Adds the project to the production queue
/// 4. Closes the menu
///
/// # Cancel
/// When cancel is clicked, simply closes the menu without adding anything.
pub fn build_menu_interaction(
    mut interaction_query: Query<
        (&Interaction, &BuildMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut cancel_query: Query<(&Interaction, &BuildMenuCancel), (Changed<Interaction>, With<Button>)>,
    mut planet_state: ResMut<PlanetViewState>,
    mut update_events: MessageWriter<crate::planet_view::types::TileUpdateEvent>,
    _game_data: Res<GameData>,
) {
    // Handle Building Selection
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(target_idx) = planet_state.build_menu_target_tile {
                // Get the selected building type from the button component
                let b_type = action.0;

                // TODO: Look up actual cost from game data
                let cost = 50;

                // Create and enqueue the production project
                planet_state.production_queue.push_back(ProductionProject {
                    project_type: ProjectType::Building(b_type),
                    total_cost: cost,
                    progress: 0,
                    target_tile_index: target_idx,
                });

                info!("Added {:?} to queue", b_type);

                if let Some(surface) = &planet_state.surface {
                    let x = target_idx % surface.row_width;
                    let y = target_idx / surface.row_width;
                    update_events.write(crate::planet_view::types::TileUpdateEvent { x, y });
                }
            }
            // Close menu after selection
            planet_state.build_menu_open = false;
            planet_state.build_menu_target_tile = None;
        }
    }

    // Handle Cancel button - just close the menu
    for (interaction, _) in &mut cancel_query {
        if *interaction == Interaction::Pressed {
            planet_state.build_menu_open = false;
            planet_state.build_menu_target_tile = None;
        }
    }
}
