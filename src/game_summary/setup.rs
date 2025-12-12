//! Setup functions for the game summary screen.

use bevy::prelude::*;

use crate::data_types::{GameData, HasDescription, Language, NamedEntity};
use crate::game_options::NewGameSettings;

use super::briefing::generate_mission_briefing;
use super::types::{GameSummaryRoot, colors};
use super::ui;

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
    ui::spawn_star_background(&mut commands);

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
            ui::spawn_main_panel(
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
