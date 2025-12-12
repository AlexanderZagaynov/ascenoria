use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::data::{HasDescription, Language, NamedEntity, Species};
use crate::game_options::types::SpeciesNameText;
use crate::game_options::types::SpeciesDescriptionText;
use crate::game_options::ui::colors;

/// Spawns the species info panel in the center.
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
                SpeciesNameText,
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
                SpeciesDescriptionText,
            ));
        });
}
