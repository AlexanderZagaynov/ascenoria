use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};

use crate::data::{HasDescription, Language, NamedEntity, Species};

use super::types::{
    BeginGameButton, SettingsButton, SpeciesListItem, SpeciesListScrollThumb, SpeciesListViewport,
};

/// Colors matching Ascendancy's new game screen.
pub mod colors {
    use bevy::prelude::*;

    /// Dark blue-black background.
    pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.06);
    /// Panel background (dark navy).
    pub const PANEL_BG: Color = Color::srgb(0.05, 0.08, 0.15);
    /// Panel border (teal).
    pub const PANEL_BORDER: Color = Color::srgb(0.2, 0.5, 0.6);
    /// Button normal.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.08, 0.12, 0.20);
    /// Button hovered.
    pub const BUTTON_HOVERED: Color = Color::srgb(0.12, 0.18, 0.28);
    /// Button pressed.
    pub const BUTTON_PRESSED: Color = Color::srgb(0.16, 0.24, 0.36);
    /// Selected item highlight.
    pub const SELECTED: Color = Color::srgb(0.15, 0.35, 0.45);
    /// Text color - cyan.
    pub const TEXT: Color = Color::srgb(0.7, 0.85, 0.9);
    /// Title text - green.
    pub const TITLE: Color = Color::srgb(0.3, 0.9, 0.5);
    /// Description text.
    pub const DESCRIPTION: Color = Color::srgb(0.6, 0.75, 0.8);
    /// Galaxy info text.
    pub const INFO: Color = Color::srgb(0.8, 0.8, 0.6);
    /// Player colors.
    pub const PLAYER_COLORS: [Color; 8] = [
        Color::srgb(0.2, 0.8, 0.3), // Green
        Color::srgb(0.8, 0.3, 0.2), // Red
        Color::srgb(0.2, 0.5, 0.9), // Blue
        Color::srgb(0.9, 0.8, 0.2), // Yellow
        Color::srgb(0.7, 0.3, 0.8), // Purple
        Color::srgb(0.9, 0.5, 0.2), // Orange
        Color::srgb(0.2, 0.8, 0.8), // Cyan
        Color::srgb(0.8, 0.4, 0.6), // Pink
    ];
}

pub fn spawn_galaxy_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(140.0), // Space for bottom bar
                },
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
        ))
        .with_children(|panel| {
            // Galaxy preview area - stars visualization
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(300.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|preview| {
                    // Placeholder stars
                    for _ in 0..30 {
                        let x = rand::random::<f32>() * 90.0 + 5.0;
                        let y = rand::random::<f32>() * 90.0 + 5.0;
                        let size = rand::random::<f32>() * 3.0 + 1.0;
                        let star_colors = [
                            Color::srgb(1.0, 0.3, 0.2),
                            Color::srgb(0.2, 0.5, 1.0),
                            Color::srgb(1.0, 0.9, 0.5),
                            Color::srgb(0.9, 0.9, 0.9),
                        ];
                        let color = star_colors[rand::random::<usize>() % star_colors.len()];

                        preview.spawn((
                            Node {
                                width: Val::Px(size),
                                height: Val::Px(size),
                                position_type: PositionType::Absolute,
                                left: Val::Percent(x),
                                top: Val::Percent(y),
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderRadius::all(Val::Px(size / 2.0)),
                        ));
                    }
                });

            // Galaxy info text
            panel.spawn((
                Text::new("Average Star Cluster\nFive Species\nNeutral Atmosphere"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::INFO),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                super::types::GalaxyInfoText,
            ));
        });
}

pub fn spawn_species_info_panel(parent: &mut ChildSpawnerCommands, species: &[Species]) {
    parent
        .spawn((
            Node {
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(140.0),
                },
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
        ))
        .with_children(|panel| {
            let first_species = species.first();
            let name = first_species
                .map(|s| s.name(Language::En))
                .unwrap_or("Unknown Species");
            let desc = first_species
                .map(|s| s.description(Language::En))
                .unwrap_or("No description available.");

            // Species name
            panel.spawn((
                Text::new(name),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(colors::TITLE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                super::types::SpeciesNameText,
            ));

            // Species portrait placeholder (circular frame)
            panel
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        border: UiRect::all(Val::Px(4.0)),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                    BorderColor::all(colors::TITLE),
                    BorderRadius::all(Val::Percent(50.0)),
                ))
                .with_children(|portrait| {
                    // Placeholder icon
                    portrait.spawn((
                        Text::new("👽"),
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));
                });

            // Home planet preview
            panel.spawn((
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::bottom(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.3, 0.6)), // Purple planet
                BorderColor::all(colors::PANEL_BORDER),
                BorderRadius::all(Val::Percent(50.0)),
            ));

            // Species description
            panel.spawn((
                Text::new(desc),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::DESCRIPTION),
                Node {
                    max_width: Val::Px(350.0),
                    ..default()
                },
                super::types::SpeciesDescriptionText,
            ));
        });
}

pub fn spawn_species_list_panel(parent: &mut ChildSpawnerCommands, species: &[Species]) {
    parent
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(140.0),
                },
                ..default()
            },
            BackgroundColor(colors::PANEL_BG),
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Player Species"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            // List Area with Scrollbar
            panel
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|row| {
                    // Left Column: Up Button + Viewport + Down Button
                    row.spawn(Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|col| {
                        // Scroll up button
                        col.spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(24.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(colors::BUTTON_NORMAL),
                            super::types::ScrollButton::Up,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("▲"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT),
                            ));
                        });

                        // Viewport
                        col.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(0.0), // Force 0 height so flex_grow works correctly
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            SpeciesListViewport,
                        ))
                        .with_children(|viewport| {
                            for (i, sp) in species.iter().enumerate() {
                                spawn_species_list_item(viewport, i, sp, i == 0);
                            }
                        });

                        // Scroll down button
                        col.spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(24.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(colors::BUTTON_NORMAL),
                            super::types::ScrollButton::Down,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("▼"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT),
                            ));
                        });
                    });

                    // Right Column: Scrollbar Track
                    row.spawn((
                        Node {
                            width: Val::Px(12.0),
                            height: Val::Percent(100.0),
                            margin: UiRect::left(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(colors::BUTTON_NORMAL), // Track color
                    ))
                    .with_children(|track| {
                        // Thumb
                        track.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(20.0), // Initial height
                                position_type: PositionType::Relative,
                                top: Val::Px(0.0),
                                ..default()
                            },
                            BackgroundColor(colors::TITLE), // Highlight color
                            SpeciesListScrollThumb,
                        ));
                    });
                });
        });
}

pub fn spawn_species_list_item(
    parent: &mut ChildSpawnerCommands,
    index: usize,
    species: &Species,
    selected: bool,
) {
    let bg_color = if selected {
        colors::SELECTED
    } else {
        colors::BUTTON_NORMAL
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(if selected {
                colors::TITLE
            } else {
                colors::PANEL_BORDER
            }),
            BorderRadius::all(Val::Px(8.0)),
            SpeciesListItem { index },
        ))
        .with_children(|item| {
            // Portrait circle
            item.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::right(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::PANEL_BG),
                BorderColor::all(colors::PANEL_BORDER),
                BorderRadius::all(Val::Percent(50.0)),
            ))
            .with_children(|circle| {
                circle.spawn((
                    Text::new("👽"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(colors::TEXT),
                ));
            });

            // Species name
            item.spawn((
                Text::new(species.name(Language::En)),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });
}

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
