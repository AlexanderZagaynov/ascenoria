use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::data::{Language, NamedEntity, Species};
use crate::game_options::types::SpeciesListItem;
use crate::game_options::ui::colors;

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
