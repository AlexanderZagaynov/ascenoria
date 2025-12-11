use bevy::{ecs::message::MessageReader, input::mouse::MouseWheel, prelude::*};

use crate::data::{GameData, HasDescription, Language, NamedEntity};
use crate::main_menu::GameState;

use super::types::*;

/// Plugin that manages the species selection screen.
pub struct GameOptionsPlugin;

impl Plugin for GameOptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOptions), setup_game_options)
            .add_systems(OnExit(GameState::GameOptions), cleanup_game_options)
            .add_systems(
                Update,
                (
                    button_system,
                    species_list_system,
                    settings_button_system,
                    begin_game_system,
                    keyboard_navigation_system,
                    species_scroll_system.after(keyboard_navigation_system),
                )
                    .run_if(in_state(GameState::GameOptions)),
            );
    }
}

fn setup_game_options(mut commands: Commands, game_data: Option<Res<GameData>>) {
    // Initialize settings if not present
    commands.init_resource::<NewGameSettings>();

    // Camera
    commands.spawn((Camera2d::default(), GameOptionsRoot));

    let species_list = game_data
        .as_ref()
        .map(|data| data.species().to_vec())
        .unwrap_or_default();

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(super::ui::colors::BACKGROUND),
            GameOptionsRoot,
        ))
        .with_children(|root| {
            // Left panel - Galaxy preview
            super::ui::spawn_galaxy_panel(root);

            // Center panel - Selected species info
            super::ui::spawn_species_info_panel(root, &species_list);

            // Right panel - Species list
            super::ui::spawn_species_list_panel(root, &species_list);
        });

    // Bottom bar with settings
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(120.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(super::ui::colors::PANEL_BG),
            GameOptionsRoot,
        ))
        .with_children(|bar| {
            super::ui::spawn_settings_buttons(bar);
            super::ui::spawn_begin_button(bar);
        });
}

fn cleanup_game_options(mut commands: Commands, query: Query<Entity, With<GameOptionsRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button interaction visual feedback.
fn button_system(
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
                    *border_color = BorderColor::all(super::ui::colors::TITLE);
                }
                Interaction::None => {
                    *border_color = BorderColor::all(if is_selected {
                        super::ui::colors::TITLE
                    } else {
                        super::ui::colors::PANEL_BORDER
                    });
                }
            }
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(super::ui::colors::BUTTON_PRESSED);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(super::ui::colors::BUTTON_HOVERED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(super::ui::colors::BUTTON_NORMAL);
            }
        }
    }
}

/// Handles species list selection.
fn species_list_system(
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
                super::ui::colors::SELECTED
            } else {
                super::ui::colors::BUTTON_NORMAL
            });
            *border = BorderColor::all(if is_selected {
                super::ui::colors::TITLE
            } else {
                super::ui::colors::PANEL_BORDER
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

/// Handles settings button clicks.
fn settings_button_system(
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
                                super::ui::colors::PANEL_BORDER
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
fn begin_game_system(
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

/// Handles keyboard navigation.
fn keyboard_navigation_system(
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
                super::ui::colors::SELECTED
            } else {
                super::ui::colors::BUTTON_NORMAL
            });
            *border = BorderColor::all(if is_selected {
                super::ui::colors::TITLE
            } else {
                super::ui::colors::PANEL_BORDER
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

/// Handles scrolling of the species list.
fn species_scroll_system(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut viewport_query: Query<
        (&mut ScrollPosition, &ComputedNode, &Children),
        With<SpeciesListViewport>,
    >,
    mut thumb_query: Query<
        &mut Node,
        (
            With<SpeciesListScrollThumb>,
            Without<SpeciesListViewport>,
            Without<SpeciesListItem>,
        ),
    >,
    button_query: Query<(&Interaction, &ScrollButton), (Changed<Interaction>, With<Button>)>,
    item_query: Query<
        (&ComputedNode, &Node),
        (
            With<SpeciesListItem>,
            Without<SpeciesListViewport>,
            Without<SpeciesListScrollThumb>,
        ),
    >,
) {
    let Some((mut scroll_pos, viewport_computed, children)) = viewport_query.iter_mut().next()
    else {
        return;
    };
    let Some(mut thumb_node) = thumb_query.iter_mut().next() else {
        return;
    };

    // Get visible height from viewport
    let visible_height = viewport_computed.size().y;

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

    let total_items = children.len() as f32;
    let total_height = total_items * item_height;
    let max_scroll = (total_height - visible_height).max(0.0);

    let current_top = scroll_pos.y;
    let mut new_top = current_top;

    // Mouse Wheel
    for event in mouse_wheel_events.read() {
        new_top -= event.y * 40.0;
    }

    // Buttons
    for (interaction, button) in &button_query {
        if *interaction == Interaction::Pressed {
            match button {
                ScrollButton::Up => new_top -= 40.0,
                ScrollButton::Down => new_top += 40.0,
            }
        }
    }

    // Clamp
    new_top = new_top.clamp(0.0, max_scroll);

    // Apply
    scroll_pos.y = new_top;

    // Update Thumb
    if total_height > 0.0 {
        let viewport_ratio = (visible_height / total_height).clamp(0.1, 1.0); // Min 10% thumb size
        let thumb_height_percent = viewport_ratio * 100.0;
        thumb_node.height = Val::Percent(thumb_height_percent);

        if max_scroll > 0.0 {
            let scroll_percent = new_top / max_scroll;
            // The track is 100%. The thumb takes up `thumb_height_percent`.
            // The available travel space is 100% - thumb_height_percent.
            let available_travel_percent = 100.0 - thumb_height_percent;
            let thumb_top_percent = scroll_percent * available_travel_percent;
            thumb_node.top = Val::Percent(thumb_top_percent);
        } else {
            thumb_node.top = Val::Percent(0.0);
        }
    }
}
