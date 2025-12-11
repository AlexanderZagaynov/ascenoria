//! UI spawn helpers for the galaxy map.
//!
//! Contains functions for spawning UI elements like panels, buttons,
//! turn indicators, and other control widgets.

use bevy::prelude::*;

use super::colors;
use super::types::PanelButton;

/// Spawn the turn indicator rings at the top of the panel.
pub fn spawn_turn_indicators(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        })
        .with_children(|row| {
            // 5 ring indicators (like in Ascendancy)
            for _ in 0..5 {
                row.spawn((
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderColor::all(colors::RING_GREEN),
                    BorderRadius::all(Val::Percent(50.0)),
                ));
            }
        });
}

/// Spawn the speed control buttons.
pub fn spawn_speed_controls(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                margin: UiRect::bottom(Val::Px(12.0)),
                column_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
        ))
        .with_children(|row| {
            // Clock + arrow speed indicators
            for label in ["⏱→", "⏱⇒"] {
                row.spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_DARK),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
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

/// Spawn a panel section button.
pub fn spawn_panel_section(
    panel: &mut ChildSpawnerCommands,
    label: &str,
    button_type: PanelButton,
) {
    panel
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                margin: UiRect::bottom(Val::Px(4.0)),
                padding: UiRect::horizontal(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::PANEL_BORDER),
            button_type,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT),
            ));

            // Icon placeholder
            btn.spawn((
                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.3, 0.4, 0.8)),
                BorderColor::all(colors::PANEL_BORDER),
            ))
            .with_children(|icon| {
                let icon_char = match button_type {
                    PanelButton::Planets => "🌍",
                    PanelButton::Ships => "🚀",
                    PanelButton::Research => "🔬",
                    PanelButton::SpecialAbility => "✨",
                    PanelButton::Species => "👽",
                    _ => "•",
                };
                icon.spawn((
                    Text::new(icon_char),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::PANEL_TEXT),
                ));
            });
        });
}

/// Spawn the bottom control buttons grid.
pub fn spawn_bottom_controls(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            margin: UiRect::top(Val::Px(8.0)),
            ..default()
        })
        .with_children(|grid| {
            // First row of 4 buttons
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("−", Color::srgb(0.5, 0.6, 0.7)),
                    ("!", Color::srgb(0.8, 0.3, 0.2)),
                    ("▲", Color::srgb(0.8, 0.5, 0.3)),
                    ("+", Color::srgb(0.4, 0.5, 0.6)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });

            // Second row of 4 buttons
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("⚙", Color::srgb(0.6, 0.5, 0.3)),
                    ("☆", Color::srgb(0.7, 0.6, 0.2)),
                    ("◎", Color::srgb(0.5, 0.3, 0.5)),
                    ("◉", Color::srgb(0.3, 0.4, 0.6)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });

            // Bottom row - speed indicators
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            })
            .with_children(|row| {
                for i in 0..5 {
                    row.spawn((
                        Node {
                            width: Val::Px(36.0),
                            height: Val::Px(24.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(colors::PANEL_DARK),
                        BorderColor::all(colors::PANEL_BORDER),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("{}", i + 1)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(colors::PANEL_TEXT_DIM),
                        ));
                    });
                }
            });
        });
}

/// Spawn a circular control button.
pub fn spawn_circular_button(parent: &mut ChildSpawnerCommands, icon: &str, bg_color: Color) {
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
            BackgroundColor(bg_color.with_alpha(0.8)),
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
