use crate::game_options::types::*;
use crate::game_options::ui;
use crate::main_menu::GameState;
use bevy::prelude::*;

/// Handles button interaction visual feedback.
pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&SpeciesListItem>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    settings: Res<NewGameSettings>,
) {
    for (interaction, mut bg_color, mut border_color, species_item) in &mut interaction_query {
        // Skip species list items - they have special handling
        if let Some(item) = species_item {
            let is_selected = item.index == settings.selected_species_index;
            match *interaction {
                Interaction::Pressed | Interaction::Hovered => {
                    *border_color = BorderColor::all(ui::colors::TITLE);
                }
                Interaction::None => {
                    *border_color = BorderColor::all(if is_selected {
                        ui::colors::TITLE
                    } else {
                        ui::colors::PANEL_BORDER
                    });
                }
            }
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(ui::colors::BUTTON_PRESSED);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(ui::colors::BUTTON_HOVERED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(ui::colors::BUTTON_NORMAL);
            }
        }
    }
}

/// Handles settings button clicks.
pub fn settings_button_system(
    interaction_query: Query<(&Interaction, &SettingsButton), (Changed<Interaction>, With<Button>)>,
    mut settings: ResMut<NewGameSettings>,
    mut color_buttons: Query<(&SettingsButton, &mut BorderColor)>,
    mut info_text: Query<&mut Text, With<GalaxyInfoText>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                SettingsButton::StarDensity => {
                    settings.star_density = (settings.star_density + 1) % 3;
                    info!(
                        "Star density: {}",
                        ["Sparse", "Average", "Dense"][settings.star_density]
                    );
                }
                SettingsButton::NumSpecies => {
                    settings.num_species = if settings.num_species >= 7 {
                        1
                    } else {
                        settings.num_species + 1
                    };
                    info!("Number of species: {}", settings.num_species);
                }
                SettingsButton::Atmosphere => {
                    settings.atmosphere = (settings.atmosphere + 1) % 3;
                    info!(
                        "Atmosphere: {}",
                        ["Neutral", "Oxygen", "Methane"][settings.atmosphere]
                    );
                }
                SettingsButton::PlayerColor(index) => {
                    settings.player_color = *index;
                    // Update border highlights
                    for (btn, mut border) in &mut color_buttons {
                        if let SettingsButton::PlayerColor(i) = btn {
                            *border = BorderColor::all(if *i == settings.player_color {
                                Color::WHITE
                            } else {
                                ui::colors::PANEL_BORDER
                            });
                        }
                    }
                    info!("Player color: {}", index);
                }
            }

            // Update info text
            let density_names = [
                "Sparse Star Cluster",
                "Average Star Cluster",
                "Dense Star Cluster",
            ];
            let species_text = match settings.num_species {
                1 => "One Species",
                2 => "Two Species",
                3 => "Three Species",
                4 => "Four Species",
                5 => "Five Species",
                6 => "Six Species",
                _ => "Seven Species",
            };
            let atmosphere_names = [
                "Neutral Atmosphere",
                "Oxygen Atmosphere",
                "Methane Atmosphere",
            ];

            for mut text in &mut info_text {
                **text = format!(
                    "{}\n{}\n{}",
                    density_names[settings.star_density],
                    species_text,
                    atmosphere_names[settings.atmosphere]
                );
            }
        }
    }
}

/// Handles begin game button.
pub fn begin_game_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BeginGameButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Proceeding to species intro!");
            next_state.set(GameState::GameSummary);
        }
    }
}
