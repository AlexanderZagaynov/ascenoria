//! Species selection screen for new game setup, inspired by Ascendancy.
//!
//! Displays species portraits, descriptions, game settings, and galaxy preview.

use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};

use crate::data::{GameData, HasDescription, Language, NamedEntity, Species};
use crate::main_menu::GameState;

/// Plugin that manages the species selection screen.
pub struct SpeciesSelectionPlugin;

impl Plugin for SpeciesSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpeciesSelection), setup_species_selection)
            .add_systems(OnExit(GameState::SpeciesSelection), cleanup_species_selection)
            .add_systems(
                Update,
                (
                    button_system,
                    species_list_system,
                    settings_button_system,
                    begin_game_system,
                    keyboard_navigation_system,
                )
                    .run_if(in_state(GameState::SpeciesSelection)),
            );
    }
}

/// Marker component for all species selection UI entities.
#[derive(Component)]
struct SpeciesSelectionRoot;

/// Resource tracking current selection state.
#[derive(Resource)]
pub struct NewGameSettings {
    /// Index of currently selected species.
    pub selected_species_index: usize,
    /// Star density setting (0 = Sparse, 1 = Average, 2 = Dense).
    pub star_density: usize,
    /// Number of AI species (1-7).
    pub num_species: usize,
    /// Atmosphere type (0 = Neutral, 1 = Oxygen, etc.).
    pub atmosphere: usize,
    /// Player color index.
    pub player_color: usize,
    /// Galaxy seed for preview.
    pub galaxy_seed: u64,
}

impl Default for NewGameSettings {
    fn default() -> Self {
        Self {
            selected_species_index: 0,
            star_density: 1,   // Average
            num_species: 5,    // Five Species
            atmosphere: 0,     // Neutral
            player_color: 0,
            galaxy_seed: rand::random(),
        }
    }
}

/// Marker for species list items.
#[derive(Component)]
struct SpeciesListItem {
    index: usize,
}

/// Marker for species name text.
#[derive(Component)]
struct SpeciesNameText;

/// Marker for species description text.
#[derive(Component)]
struct SpeciesDescriptionText;

/// Marker for galaxy info text.
#[derive(Component)]
struct GalaxyInfoText;

/// Settings buttons.
#[derive(Component, Debug, Clone, Copy)]
enum SettingsButton {
    StarDensity,
    NumSpecies,
    Atmosphere,
    PlayerColor(usize),
}

/// Begin game button.
#[derive(Component)]
struct BeginGameButton;

/// Colors matching Ascendancy's new game screen.
mod colors {
    use bevy::prelude::*;

    /// Dark blue-black background.
    pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.06);
    /// Panel background (dark navy).
    pub const PANEL_BG: Color = Color::srgb(0.05, 0.08, 0.15);
    /// Panel border (teal).
    pub const PANEL_BORDER: Color = Color::srgb(0.2, 0.5, 0.6);
    /// Button normal.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.08, 0.12, 0.20);
    /// Button hovered.
    pub const BUTTON_HOVERED: Color = Color::srgb(0.12, 0.18, 0.28);
    /// Button pressed.
    pub const BUTTON_PRESSED: Color = Color::srgb(0.16, 0.24, 0.36);
    /// Selected item highlight.
    pub const SELECTED: Color = Color::srgb(0.15, 0.35, 0.45);
    /// Text color - cyan.
    pub const TEXT: Color = Color::srgb(0.7, 0.85, 0.9);
    /// Title text - green.
    pub const TITLE: Color = Color::srgb(0.3, 0.9, 0.5);
    /// Description text.
    pub const DESCRIPTION: Color = Color::srgb(0.6, 0.75, 0.8);
    /// Galaxy info text.
    pub const INFO: Color = Color::srgb(0.8, 0.8, 0.6);
    /// Player colors.
    pub const PLAYER_COLORS: [Color; 8] = [
        Color::srgb(0.2, 0.8, 0.3),  // Green
        Color::srgb(0.8, 0.3, 0.2),  // Red
        Color::srgb(0.2, 0.5, 0.9),  // Blue
        Color::srgb(0.9, 0.8, 0.2),  // Yellow
        Color::srgb(0.7, 0.3, 0.8),  // Purple
        Color::srgb(0.9, 0.5, 0.2),  // Orange
        Color::srgb(0.2, 0.8, 0.8),  // Cyan
        Color::srgb(0.8, 0.4, 0.6),  // Pink
    ];
}

fn setup_species_selection(
    mut commands: Commands,
    game_data: Option<Res<GameData>>,
) {
    // Initialize settings if not present
    commands.init_resource::<NewGameSettings>();

    // Camera
    commands.spawn((Camera2d::default(), SpeciesSelectionRoot));

    let species_list: Vec<Species> = game_data
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
            BackgroundColor(colors::BACKGROUND),
            SpeciesSelectionRoot,
        ))
        .with_children(|root| {
            // Left panel - Galaxy preview
            spawn_galaxy_panel(root);

            // Center panel - Selected species info
            spawn_species_info_panel(root, &species_list);

            // Right panel - Species list
            spawn_species_list_panel(root, &species_list);
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
            BackgroundColor(colors::PANEL_BG),
            SpeciesSelectionRoot,
        ))
        .with_children(|bar| {
            spawn_settings_buttons(bar);
            spawn_begin_button(bar);
        });
}

fn spawn_galaxy_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(140.0), // Space for bottom bar
                },
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
        ))
        .with_children(|panel| {
            // Galaxy preview area - stars visualization
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(300.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND),
                    BorderColor::all(colors::PANEL_BORDER),
                ))
                .with_children(|preview| {
                    // Placeholder stars
                    for _ in 0..30 {
                        let x = rand::random::<f32>() * 90.0 + 5.0;
                        let y = rand::random::<f32>() * 90.0 + 5.0;
                        let size = rand::random::<f32>() * 3.0 + 1.0;
                        let star_colors = [
                            Color::srgb(1.0, 0.3, 0.2),
                            Color::srgb(0.2, 0.5, 1.0),
                            Color::srgb(1.0, 0.9, 0.5),
                            Color::srgb(0.9, 0.9, 0.9),
                        ];
                        let color = star_colors[rand::random::<usize>() % star_colors.len()];

                        preview.spawn((
                            Node {
                                width: Val::Px(size),
                                height: Val::Px(size),
                                position_type: PositionType::Absolute,
                                left: Val::Percent(x),
                                top: Val::Percent(y),
                                ..default()
                            },
                            BackgroundColor(color),
                            BorderRadius::all(Val::Px(size / 2.0)),
                        ));
                    }
                });

            // Galaxy info text
            panel.spawn((
                Text::new("Average Star Cluster\nFive Species\nNeutral Atmosphere"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::INFO),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
                GalaxyInfoText,
            ));
        });
}

fn spawn_species_info_panel(parent: &mut ChildSpawnerCommands, species: &[Species]) {
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

fn spawn_species_list_panel(parent: &mut ChildSpawnerCommands, species: &[Species]) {
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

            // Scroll up button
            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_NORMAL),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("▲"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));
                });

            // Species list (scrollable area)
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                ))
                .with_children(|list| {
                    for (i, sp) in species.iter().enumerate() {
                        spawn_species_list_item(list, i, sp, i == 0);
                    }
                });

            // Scroll down button
            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_NORMAL),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("▼"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));
                });
        });
}

fn spawn_species_list_item(
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
            BorderColor::all(if selected { colors::TITLE } else { colors::PANEL_BORDER }),
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

fn spawn_settings_buttons(parent: &mut ChildSpawnerCommands) {
    // Star Density button
    spawn_setting_button(parent, "Star Density", SettingsButton::StarDensity);

    // Species count button
    spawn_setting_button(parent, "Species", SettingsButton::NumSpecies);

    // Atmosphere button
    spawn_setting_button(parent, "Atmosphere", SettingsButton::Atmosphere);

    // Player Color selector
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|col| {
            col.spawn((
                Text::new("Player Color"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Color buttons row
            col.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|row| {
                for i in 0..8 {
                    row.spawn((
                        Button,
                        Node {
                            width: Val::Px(25.0),
                            height: Val::Px(25.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(colors::PLAYER_COLORS[i]),
                        BorderColor::all(if i == 0 { Color::WHITE } else { colors::PANEL_BORDER }),
                        SettingsButton::PlayerColor(i),
                    ));
                }
            });
        });
}

fn spawn_setting_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    button_type: SettingsButton,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|col| {
            col.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            col.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::BUTTON_NORMAL),
                BorderColor::all(colors::PANEL_BORDER),
                BorderRadius::all(Val::Px(8.0)),
                button_type,
            ))
            .with_children(|btn| {
                // Icon placeholder
                btn.spawn((
                    Text::new("⭐"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(colors::TEXT),
                ));
            });
        });
}

fn spawn_begin_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::TITLE),
            BorderRadius::all(Val::Px(12.0)),
            BeginGameButton,
        ))
        .with_children(|btn| {
            // Galaxy preview mini
            btn.spawn((
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(40.0),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|mini| {
                mini.spawn((
                    Text::new("🌌"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(colors::TEXT),
                ));
            });

            btn.spawn((
                Text::new("Begin New Game"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });
}

fn cleanup_species_selection(mut commands: Commands, query: Query<Entity, With<SpeciesSelectionRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button interaction visual feedback.
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, Option<&SpeciesListItem>),
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
                    *border_color = BorderColor::all(colors::TITLE);
                }
                Interaction::None => {
                    *border_color = BorderColor::all(
                        if is_selected { colors::TITLE } else { colors::PANEL_BORDER }
                    );
                }
            }
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(colors::BUTTON_PRESSED);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVERED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
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
            *bg = BackgroundColor(if is_selected { colors::SELECTED } else { colors::BUTTON_NORMAL });
            *border = BorderColor::all(if is_selected { colors::TITLE } else { colors::PANEL_BORDER });
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
    interaction_query: Query<
        (&Interaction, &SettingsButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: ResMut<NewGameSettings>,
    mut color_buttons: Query<(&SettingsButton, &mut BorderColor)>,
    mut info_text: Query<&mut Text, With<GalaxyInfoText>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                SettingsButton::StarDensity => {
                    settings.star_density = (settings.star_density + 1) % 3;
                    info!("Star density: {}", ["Sparse", "Average", "Dense"][settings.star_density]);
                }
                SettingsButton::NumSpecies => {
                    settings.num_species = if settings.num_species >= 7 { 1 } else { settings.num_species + 1 };
                    info!("Number of species: {}", settings.num_species);
                }
                SettingsButton::Atmosphere => {
                    settings.atmosphere = (settings.atmosphere + 1) % 3;
                    info!("Atmosphere: {}", ["Neutral", "Oxygen", "Methane"][settings.atmosphere]);
                }
                SettingsButton::PlayerColor(index) => {
                    settings.player_color = *index;
                    // Update border highlights
                    for (btn, mut border) in &mut color_buttons {
                        if let SettingsButton::PlayerColor(i) = btn {
                            *border = BorderColor::all(
                                if *i == settings.player_color { Color::WHITE } else { colors::PANEL_BORDER }
                            );
                        }
                    }
                    info!("Player color: {}", index);
                }
            }

            // Update info text
            let density_names = ["Sparse Star Cluster", "Average Star Cluster", "Dense Star Cluster"];
            let species_text = match settings.num_species {
                1 => "One Species",
                2 => "Two Species",
                3 => "Three Species",
                4 => "Four Species",
                5 => "Five Species",
                6 => "Six Species",
                _ => "Seven Species",
            };
            let atmosphere_names = ["Neutral Atmosphere", "Oxygen Atmosphere", "Methane Atmosphere"];

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
            info!("Beginning new game!");
            next_state.set(GameState::InGame);
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
) {
    let species_count = game_data.as_ref().map(|d| d.species().len()).unwrap_or(0);
    if species_count == 0 {
        return;
    }

    let mut selection_changed = false;

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    } else if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::InGame);
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
            *bg = BackgroundColor(if is_selected { colors::SELECTED } else { colors::BUTTON_NORMAL });
            *border = BorderColor::all(if is_selected { colors::TITLE } else { colors::PANEL_BORDER });
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
