//! UI spawning helpers for the star system view.
//!
//! Contains functions to spawn the UI panel, buttons, and labels.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::galaxy::{Galaxy, StarSystem};

use super::types::{PlanetVisual, StarSystemRoot, SystemPanelButton, colors};

/// Spawn the right-side UI panel.
pub fn spawn_ui_panel(commands: &mut Commands, galaxy: &Galaxy, system_index: usize) {
    let system = galaxy.systems.get(system_index);
    let system_name = system.map(|s| s.name.as_str()).unwrap_or("Unknown");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            StarSystemRoot,
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
                    // System name header
                    spawn_system_header(panel, system_name);

                    // Navigation button row (like in screenshot)
                    spawn_navigation_buttons(panel);

                    // Planet info area
                    spawn_planet_info_area(panel, system);

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

fn spawn_system_header(panel: &mut ChildSpawnerCommands, system_name: &str) {
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::PANEL_BORDER),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new(system_name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::STAR_LABEL),
            ));

            header.spawn((
                Text::new("White Medium"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT_DIM),
            ));
        });
}

fn spawn_navigation_buttons(panel: &mut ChildSpawnerCommands) {
    // Row of icon buttons (like the screenshot shows)
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(8.0)),
            column_gap: Val::Px(4.0),
            ..default()
        },))
        .with_children(|row| {
            // Navigation icons (simplified representations)
            for (icon, _button) in [
                ("🌍", SystemPanelButton::GotoPlanet),
                ("➡", SystemPanelButton::SendFleet),
                ("🔍", SystemPanelButton::SystemInfo),
                ("➤", SystemPanelButton::BuildShip),
            ] {
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(45.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_ICON_BG),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::PANEL_TEXT),
                    ));
                });
            }
        });

    // Second row of navigation icons
    panel
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(12.0)),
            column_gap: Val::Px(4.0),
            ..default()
        },))
        .with_children(|row| {
            for icon in ["📍", "🚀", "⚙", "📋"] {
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(45.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_ICON_BG),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(icon),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::PANEL_TEXT),
                    ));
                });
            }
        });
}

fn spawn_planet_info_area(panel: &mut ChildSpawnerCommands, system: Option<&StarSystem>) {
    // Planet preview area (large image-like box)
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.08, 0.12)),
            BorderColor::all(colors::SELECTION_GREEN),
        ))
        .with_children(|preview| {
            // Planet name label
            if let Some(sys) = system {
                if let Some(planet) = sys.planets.first() {
                    preview.spawn((
                        Text::new(format!("{} I", sys.name.replace("System-", "Icarus"))),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::PLANET_LABEL),
                    ));

                    // Placeholder planet representation
                    preview.spawn((
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(
                            PlanetVisual::from_surface_type(&planet.surface_type_id)
                                .primary_color(),
                        ),
                        BorderRadius::all(Val::Percent(50.0)),
                    ));
                }
            }
        });
}

fn spawn_bottom_controls(panel: &mut ChildSpawnerCommands) {
    // Grid of circular control buttons (matching Ascendancy style)
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            margin: UiRect::top(Val::Px(8.0)),
            ..default()
        })
        .with_children(|grid| {
            // First row
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("−", Color::srgb(0.4, 0.5, 0.6)),
                    ("⚠", Color::srgb(0.8, 0.3, 0.2)),
                    ("▲", Color::srgb(0.85, 0.55, 0.25)),
                    ("+", Color::srgb(0.4, 0.5, 0.6)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });

            // Second row
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("⌀", Color::srgb(0.6, 0.4, 0.3)),
                    ("☆", Color::srgb(0.7, 0.6, 0.2)),
                    ("◎", Color::srgb(0.45, 0.55, 0.65)),
                    ("◈", Color::srgb(0.35, 0.45, 0.55)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });
        });
}

fn spawn_circular_button(parent: &mut ChildSpawnerCommands, icon: &str, bg_color: Color) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(44.0),
                height: Val::Px(44.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color.with_alpha(0.85)),
            BorderColor::all(colors::PANEL_BORDER),
            BorderRadius::all(Val::Percent(50.0)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(icon),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT),
            ));
        });
}

/// Spawn system labels (player icons in corners).
pub fn spawn_system_label(commands: &mut Commands, _system_name: &str) {
    // Player species icon in corner
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
            BorderColor::all(colors::SELECTION_GREEN),
            StarSystemRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::SELECTION_GREEN),
            ));
        });

    // Same icon in top-right (mirrored like in Ascendancy)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(230.0), // Account for panel width
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::SELECTION_GREEN),
            StarSystemRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::SELECTION_GREEN),
            ));
        });
}
