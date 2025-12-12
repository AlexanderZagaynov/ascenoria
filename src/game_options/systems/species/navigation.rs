use crate::data::{GameData, HasDescription, Language, NamedEntity};
use crate::game_options::types::*;
use crate::game_options::ui;
use crate::main_menu::GameState;
use bevy::prelude::*;

/// Handles keyboard navigation.
pub fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<NewGameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut species_items: Query<(&SpeciesListItem, &mut BackgroundColor, &mut BorderColor)>,
    mut name_text: Query<&mut Text, (With<SpeciesNameText>, Without<SpeciesDescriptionText>)>,
    mut desc_text: Query<&mut Text, (With<SpeciesDescriptionText>, Without<SpeciesNameText>)>,
    game_data: Option<Res<GameData>>,
    mut viewport_query: Query<
        (&mut ScrollPosition, &ComputedNode, &Children),
        With<SpeciesListViewport>,
    >,
    item_query: Query<
        (&ComputedNode, &Node),
        (With<SpeciesListItem>, Without<SpeciesListViewport>),
    >,
) {
    let species_count = game_data.as_ref().map(|d| d.species().len()).unwrap_or(0);
    if species_count == 0 {
        return;
    }

    let mut selection_changed = false;

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    } else if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::GameSummary);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        if settings.selected_species_index > 0 {
            settings.selected_species_index -= 1;
            selection_changed = true;
        }
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        if settings.selected_species_index < species_count - 1 {
            settings.selected_species_index += 1;
            selection_changed = true;
        }
    }

    if selection_changed {
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

        // Scroll into view
        if let Some((mut scroll_pos, viewport_computed, children)) =
            viewport_query.iter_mut().next()
        {
            // Calculate item height dynamically
            let item_height = if let Some(first_child) = children.first() {
                if let Ok((computed, style)) = item_query.get(*first_child) {
                    let h = computed.size().y;
                    let margin = match style.margin.bottom {
                        Val::Px(v) => v,
                        _ => 0.0,
                    };
                    let total = h + margin;
                    if h > 0.0 { total } else { 85.0 }
                } else {
                    85.0
                }
            } else {
                85.0
            };

            let visible_height = viewport_computed.size().y;
            let total_items = children.len() as f32;
            let total_height = total_items * item_height;
            let max_scroll = (total_height - visible_height).max(0.0);

            let current_scroll = scroll_pos.y;

            let selected_index = settings.selected_species_index as f32;
            let item_top = selected_index * item_height;
            let item_bottom = item_top + item_height;

            // Visible range: [current_scroll, current_scroll + visible_height]
            let viewport_top = current_scroll;
            let viewport_bottom = viewport_top + visible_height;

            let mut new_scroll = current_scroll;

            if item_top < viewport_top {
                // Item is above viewport
                new_scroll = item_top;
            } else if item_bottom > viewport_bottom {
                // Item is below viewport
                new_scroll = item_bottom - visible_height;
            }

            // Clamp
            new_scroll = new_scroll.clamp(0.0, max_scroll);

            scroll_pos.y = new_scroll;
        }
    }
}
