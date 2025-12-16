use bevy::prelude::*;
// use bevy::ecs::hierarchy::DespawnRecursiveExt;
use crate::data_types::GameData;
use crate::planet_data::BuildingType;
use crate::planet_view::types::{PlanetViewState, ProductionProject, ProjectType};

#[derive(Component)]
pub struct BuildMenuRoot;

#[derive(Component)]
pub struct BuildMenuAction(pub BuildingType);

#[derive(Component)]
pub struct BuildMenuCancel;

pub fn update_build_menu(
    mut commands: Commands,
    planet_state: Res<PlanetViewState>,
    menu_query: Query<Entity, With<BuildMenuRoot>>,
    game_data: Res<GameData>,
) {
    let is_open = planet_state.build_menu_open;
    let has_menu = !menu_query.is_empty();

    if is_open && !has_menu {
        spawn_build_menu(&mut commands, &game_data);
    } else if !is_open && has_menu {
        for entity in &menu_query {
            commands.entity(entity).despawn();
        }
    }
}

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

pub fn build_menu_interaction(
    mut interaction_query: Query<
        (&Interaction, &BuildMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut cancel_query: Query<(&Interaction, &BuildMenuCancel), (Changed<Interaction>, With<Button>)>,
    mut planet_state: ResMut<PlanetViewState>,
    _game_data: Res<GameData>,
) {
    // Handle Building Selection
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(target_idx) = planet_state.build_menu_target_tile {
                // Add to queue
                let b_type = action.0;

                // Placeholder cost
                let cost = 50;

                planet_state.production_queue.push_back(ProductionProject {
                    project_type: ProjectType::Building(b_type),
                    total_cost: cost,
                    progress: 0,
                    target_tile_index: target_idx,
                });

                info!("Added {:?} to queue", b_type);
            }
            planet_state.build_menu_open = false;
            planet_state.build_menu_target_tile = None;
        }
    }

    // Handle Cancel
    for (interaction, _) in &mut cancel_query {
        if *interaction == Interaction::Pressed {
            planet_state.build_menu_open = false;
            planet_state.build_menu_target_tile = None;
        }
    }
}
