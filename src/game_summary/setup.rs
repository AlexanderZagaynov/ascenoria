//! Setup functions for the game summary screen.

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::data::{GameData, HasDescription, Language, NamedEntity};
use crate::game_options::NewGameSettings;

use super::briefing::generate_mission_briefing;
use super::types::{
    BackButton, ContinueButton, GameSummaryRoot, StarBackground, SummaryScrollContent,
    SummaryScrollViewport, colors,
};

/// Sets up the game summary screen.
pub fn setup_game_summary(
    mut commands: Commands,
    settings: Res<NewGameSettings>,
    game_data: Option<Res<GameData>>,
) {
    // Camera
    commands.spawn((Camera2d::default(), GameSummaryRoot));

    // Get selected species info
    let (species_name, species_description, species_id) = game_data
        .as_ref()
        .and_then(|data| data.species().get(settings.selected_species_index))
        .map(|s| {
            (
                s.name(Language::En).to_string(),
                s.description(Language::En).to_string(),
                s.id.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown Species".to_string(),
                "No description available.".to_string(),
                "unknown".to_string(),
            )
        });

    // Generate mission briefing text based on species
    let mission_briefing = generate_mission_briefing(&species_name, &species_id);

    // Spawn star background
    spawn_star_background(&mut commands);

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
            GameSummaryRoot,
        ))
        .with_children(|parent| {
            // Main content panel
            spawn_main_panel(
                parent,
                &species_name,
                &species_description,
                &mission_briefing,
            );

            // Hint text at bottom
            parent.spawn((
                Text::new("Press ENTER to continue or ESC to return"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::HINT_TEXT),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

/// Spawns the main content panel.
fn spawn_main_panel(
    parent: &mut ChildSpawnerCommands,
    species_name: &str,
    species_description: &str,
    mission_briefing: &str,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(1000.0),
                height: Val::Px(700.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(30.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::PANEL_BG),
            BorderColor::all(colors::PANEL_BORDER),
            BorderRadius::all(Val::Px(12.0)),
        ))
        .with_children(|panel| {
            // Title section
            spawn_title_section(panel, species_name);

            // Content area (portrait + text)
            spawn_content_area(panel, species_description, mission_briefing);

            // Bottom buttons
            spawn_button_area(panel);
        });
}

/// Spawns the title section with species name.
fn spawn_title_section(panel: &mut ChildSpawnerCommands, species_name: &str) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::bottom(Val::Px(10.0)),
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        })
        .with_children(|title_area| {
            // Species name (large title)
            title_area.spawn((
                Text::new(species_name),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(colors::TITLE),
            ));

            // Subtitle
            title_area.spawn((
                Text::new("Game Summary"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::SUBTITLE),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
            ));
        });
}

/// Spawns the content area with portrait and text.
fn spawn_content_area(
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

/// Spawns the bottom button area.
fn spawn_button_area(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        })
        .with_children(|button_area| {
            // Back button
            spawn_button(button_area, "← Back to Options", BackButton);

            // Continue button
            spawn_button(button_area, "Continue", ContinueButton);
        });
}

/// Spawns a button with the given label and marker component.
fn spawn_button<M: Component>(button_area: &mut ChildSpawnerCommands, label: &str, marker: M) {
    button_area
        .spawn((
            Button,
            Node {
                padding: UiRect::new(Val::Px(30.0), Val::Px(30.0), Val::Px(15.0), Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::PANEL_BORDER),
            BorderRadius::all(Val::Px(6.0)),
            marker,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });
}

/// Spawns decorative star background.
fn spawn_star_background(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            StarBackground,
            GameSummaryRoot,
            ZIndex(-1),
        ))
        .with_children(|parent| {
            // Spawn some decorative "stars" as small nodes
            let star_positions = [
                (10.0, 15.0, 2.0),
                (25.0, 8.0, 3.0),
                (40.0, 22.0, 2.0),
                (55.0, 5.0, 2.5),
                (70.0, 18.0, 2.0),
                (85.0, 12.0, 3.0),
                (15.0, 75.0, 2.0),
                (30.0, 85.0, 2.5),
                (50.0, 78.0, 2.0),
                (65.0, 90.0, 3.0),
                (80.0, 82.0, 2.0),
                (92.0, 70.0, 2.5),
                (5.0, 45.0, 2.0),
                (95.0, 40.0, 2.0),
                (88.0, 55.0, 2.5),
                (8.0, 60.0, 2.0),
            ];

            for (x, y, size) in star_positions {
                parent.spawn((
                    Node {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.8, 0.8, 0.9, 0.6)),
                    BorderRadius::all(Val::Px(size / 2.0)),
                ));
            }
        });
}
