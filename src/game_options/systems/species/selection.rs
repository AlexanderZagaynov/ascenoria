use crate::data_types::{GameData, HasDescription, Language, NamedEntity};
use crate::game_options::types::*;
use crate::game_options::ui;
use bevy::prelude::*;

/// Handles species list selection.
pub fn species_list_system(
    interaction_query: Query<
        (&Interaction, &SpeciesListItem),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: ResMut<NewGameSettings>,
    mut species_items: Query<(&SpeciesListItem, &mut BackgroundColor, &mut BorderColor)>,
    mut name_text: Query<&mut Text, (With<SpeciesNameText>, Without<SpeciesDescriptionText>)>,
    mut desc_text: Query<&mut Text, (With<SpeciesDescriptionText>, Without<SpeciesNameText>)>,
    game_data: Option<Res<GameData>>,
) {
    let mut selected_changed = false;

    for (interaction, item) in &interaction_query {
        if *interaction == Interaction::Pressed && item.index != settings.selected_species_index {
            settings.selected_species_index = item.index;
            selected_changed = true;
        }
    }

    if selected_changed {
        // Update visual selection
        for (item, mut bg, mut border) in &mut species_items {
            let is_selected = item.index == settings.selected_species_index;
            *bg = BackgroundColor(if is_selected {
                ui::colors::SELECTED
            } else {
                ui::colors::BUTTON_NORMAL
            });
            *border = BorderColor::all(if is_selected {
                ui::colors::TITLE
            } else {
                ui::colors::PANEL_BORDER
            });
        }

        // Update species info display
        if let Some(data) = &game_data {
            if let Some(species) = data.species().get(settings.selected_species_index) {
                for mut text in &mut name_text {
                    **text = species.name(Language::En).to_string();
                }
                for mut text in &mut desc_text {
                    **text = species.description(Language::En).to_string();
                }
            }
        }
    }
}
