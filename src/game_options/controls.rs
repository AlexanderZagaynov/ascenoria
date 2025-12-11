//! Control buttons UI for the game options screen.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::game_options::types::{BeginGameButton, SettingsButton};
use crate::game_options::ui::colors;

/// Spawns all settings buttons (star density, species count, atmosphere, player color).
pub fn spawn_settings_buttons(parent: &mut ChildSpawnerCommands) {
    // Star Density button
    spawn_setting_button(parent, "Star Density", SettingsButton::StarDensity);

    // Species count button
    spawn_setting_button(parent, "Species", SettingsButton::NumSpecies);

    // Atmosphere button
    spawn_setting_button(parent, "Atmosphere", SettingsButton::Atmosphere);

    // Player Color selector
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Player Color"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Color buttons row
            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                for i in 0..8 {
                    row.spawn((
                        Button,
                        Node {
                            width: Val::Px(25.0),
                            height: Val::Px(25.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(colors::PLAYER_COLORS[i]),
                        BorderColor::all(if i == 0 {
                            Color::WHITE
                        } else {
                            colors::PANEL_BORDER
                        }),
                        SettingsButton::PlayerColor(i),
                    ));
                }
            });
        });
}

/// Spawns a single setting button with label.
pub fn spawn_setting_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    button_type: SettingsButton,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            col.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::BUTTON_NORMAL),
                BorderColor::all(colors::PANEL_BORDER),
                BorderRadius::all(Val::Px(8.0)),
                button_type,
            ))
            .with_children(|btn| {
                // Icon placeholder
                btn.spawn((
                    Text::new("⭐"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(colors::TEXT),
                ));
            });
        });
}

/// Spawns the "Begin New Game" button.
pub fn spawn_begin_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::TITLE),
            BorderRadius::all(Val::Px(12.0)),
            BeginGameButton,
        ))
        .with_children(|btn| {
            // Galaxy preview mini
            btn.spawn(Node {
                width: Val::Px(50.0),
                height: Val::Px(40.0),
                margin: UiRect::bottom(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|mini| {
                mini.spawn((
                    Text::new("🌌"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(colors::TEXT),
                ));
            });

            btn.spawn((
                Text::new("Begin New Game"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });
}
