use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use super::super::types::{SummaryScrollContent, SummaryScrollViewport, colors};

/// Spawns the content area with portrait and text.
pub fn spawn_content_area(
    panel: &mut ChildSpawnerCommands,
    species_description: &str,
    mission_briefing: &str,
) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(30.0),
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        })
        .with_children(|content| {
            // Left side - Portrait
            spawn_portrait_area(content);

            // Right side - Text content
            spawn_text_area(content, species_description, mission_briefing);
        });
}

/// Spawns the portrait area on the left side.
fn spawn_portrait_area(content: &mut ChildSpawnerCommands) {
    content
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::PORTRAIT_BG),
            BorderColor::all(colors::PORTRAIT_BORDER),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|portrait_area| {
            // Portrait placeholder
            portrait_area
                .spawn((
                    Node {
                        width: Val::Px(240.0),
                        height: Val::Px(320.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.12, 0.15)),
                    BorderColor::all(colors::PANEL_BORDER),
                    BorderRadius::all(Val::Px(4.0)),
                ))
                .with_children(|placeholder| {
                    // Placeholder text
                    placeholder.spawn((
                        Text::new("[Species Portrait]"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::HINT_TEXT),
                    ));
                });

            // "Leader" label below portrait
            portrait_area.spawn((
                Text::new("Species Leader"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::SUBTITLE),
                Node {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
            ));
        });
}

/// Spawns the scrollable text area on the right side.
fn spawn_text_area(
    content: &mut ChildSpawnerCommands,
    species_description: &str,
    mission_briefing: &str,
) {
    content
        .spawn((
            Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ScrollPosition::default(),
            SummaryScrollViewport,
        ))
        .with_children(|viewport| {
            viewport
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    SummaryScrollContent,
                ))
                .with_children(|text_area| {
                    // Mission briefing section
                    spawn_text_section(text_area, "MISSION BRIEFING", mission_briefing);

                    // Species lore section
                    spawn_text_section(text_area, "SPECIES PROFILE", species_description);
                });
        });
}

/// Spawns a text section with title and content.
fn spawn_text_section(text_area: &mut ChildSpawnerCommands, title: &str, content: &str) {
    text_area
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(10.0)),
            ..default()
        })
        .with_children(|section| {
            section.spawn((
                Text::new(title),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TITLE),
            ));

            section.spawn((
                Text::new(content),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}
