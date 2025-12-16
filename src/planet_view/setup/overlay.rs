//! 2D UI overlay for the Planet View screen.
//!
//! Creates the HUD elements that appear on top of the 3D scene:
//! - Top bar with resource counters and turn number
//! - Victory message (hidden until triggered)
//! - Bottom bar with End Turn button

// use crate::planet_data::BuildingType;
use crate::planet_view::types::{PlanetViewRoot, UIAction};
use bevy::core_pipeline::core_2d::graph::Core2d;
use bevy::render::camera::CameraRenderGraph;
use bevy::prelude::*;

/// Set up the 2D UI overlay.
///
/// # Layout
/// ```text
/// ┌────────────────────────────────────────────────────┐
/// │ Turn: 1  Food: 0  Housing: 0  Prod: 0  Science: 0  │  ← Top Bar
/// ├────────────────────────────────────────────────────┤
/// │                                                    │
/// │                  3D Scene Area                     │
/// │                                                    │
/// │         ┌──────────────────────────┐               │
/// │         │  VICTORY! (hidden)       │               │  ← Victory Message
/// │         │  [Return to Menu]        │               │
/// │         └──────────────────────────┘               │
/// │                                                    │
/// ├────────────────────────────────────────────────────┤
/// │              [End Turn]                            │  ← Bottom Bar
/// └────────────────────────────────────────────────────┘
/// ```
///
/// # Components
/// - `PlanetViewRoot` - Marker for cleanup on screen exit
/// - `VictoryMessage` - Hidden message shown when victory condition met
/// - `UIAction::EndTurn` - Button to advance the turn
/// - `UIAction::Quit` - Button to return to main menu
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
