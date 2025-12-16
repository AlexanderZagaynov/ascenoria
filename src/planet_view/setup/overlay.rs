use crate::planet_data::BuildingType;
use crate::planet_view::types::{PlanetViewRoot, UIAction};
use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::render::camera::CameraRenderGraph;
use bevy::prelude::*;

/// Set up the 2D UI overlay.
pub fn setup_ui_overlay(commands: &mut Commands) {
    // 2D Camera for UI overlay
    commands.spawn((
        Camera2d::default(),
        CameraRenderGraph::new(Core2d),
        PlanetViewRoot,
    ));

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            PlanetViewRoot,
        ))
        .with_children(|root| {
            // Top Bar: Resources
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(Color::BLACK.with_alpha(0.8)),
            ))
            .with_children(|top| {
                spawn_text(top, "Turn: 1");
                spawn_text(top, "Food: 0");
                spawn_text(top, "Housing: 0");
                spawn_text(top, "Prod: 0");
                spawn_text(top, "Science: 0");
                spawn_text(top, "Research: 0/100"); // Placeholder
            });

            // Center: Victory Message (Hidden by default)
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    // translate: Transform::from_xyz(-50.0, -50.0, 0.0), // CSS translate not available in Bevy UI yet?
                    // We'll just center it roughly or use margins
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Px(20.0)),
                    display: Display::None, // Hidden initially
                    ..default()
                },
                BackgroundColor(Color::BLACK.with_alpha(0.9)),
                crate::planet_view::types::VictoryMessage,
            ))
            .with_children(|msg| {
                msg.spawn((
                    Text::new("VICTORY! All cells occupied."),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Return to Menu button
                msg.spawn((
                    Button,
                    Node {
                        margin: UiRect::top(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .insert(UIAction::Quit)
                .with_children(|btn| {
                    btn.spawn((Text::new("Return to Menu"), TextColor(Color::WHITE)));
                });
            });

            // Bottom Bar: Controls
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK.with_alpha(0.8)),
            ))
            .with_children(|bottom| {
                spawn_building_button(bottom, "Farm", BuildingType::Farm);
                spawn_building_button(bottom, "Habitat", BuildingType::Habitat);
                spawn_building_button(bottom, "Factory", BuildingType::Factory);
                spawn_building_button(bottom, "Lab", BuildingType::Laboratory);
                spawn_building_button(bottom, "Passage", BuildingType::Passage);
                spawn_building_button(bottom, "Terraform", BuildingType::Terraformer);

                // Spacer
                bottom.spawn(Node {
                    width: Val::Px(20.0),
                    ..default()
                });

                // End Turn
                bottom
                    .spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 0.5, 0.0)),
                    ))
                    .insert(UIAction::EndTurn)
                    .with_children(|btn| {
                        btn.spawn((Text::new("End Turn"), TextColor(Color::WHITE)));
                    });
            });
        });
}

fn spawn_text(parent: &mut ChildSpawnerCommands, text: &str) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn spawn_building_button(parent: &mut ChildSpawnerCommands, label: &str, building: BuildingType) {
    parent
        .spawn((
            Button,
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .insert(UIAction::SelectBuilding(building))
        .with_children(|btn| {
            btn.spawn((Text::new(label), TextColor(Color::WHITE)));
        });
}
