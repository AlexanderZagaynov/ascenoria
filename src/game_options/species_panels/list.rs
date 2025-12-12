use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::data::{Language, NamedEntity, Species};
use crate::game_options::types::{
    ScrollButton, SpeciesListItem, SpeciesListScrollThumb, SpeciesListViewport,
};
use crate::game_options::ui::colors;

/// Spawns the species list panel on the right side.
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
                        spawn_scroll_button(col, "▲", ScrollButton::Up, true);

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
                        spawn_scroll_button(col, "▼", ScrollButton::Down, false);
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

/// Spawns a scroll button (up or down).
fn spawn_scroll_button(
    col: &mut ChildSpawnerCommands,
    icon: &str,
    button_type: ScrollButton,
    is_up: bool,
) {
    col.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(24.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: if is_up {
                UiRect::bottom(Val::Px(5.0))
            } else {
                UiRect::top(Val::Px(5.0))
            },
            ..default()
        },
        BackgroundColor(colors::BUTTON_NORMAL),
        button_type,
    ))
    .with_children(|btn| {
        btn.spawn((
            Text::new(icon),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT),
        ));
    });
}

/// Spawns a single species list item.
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
